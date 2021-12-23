use core::panic;
use std::str::Chars;

use super::super::utils::*;

fn to_binary(input: &str) -> String {
    let input = input.trim();
    let mut bin = Vec::new();
    for i in input.chars() {
        let s = format!("{:04b}", i.to_digit(16).unwrap());
        // println!("{:04b}", i.to_digit(16).unwrap());
        bin.push(s);
    }
    bin.concat()
}

pub fn day_16() {
    let input = get_input(16);
    let bin = to_binary(&input);
    println!("Part 1: {}\n", part_one(&bin));
    println!("Part 2: {}", part_two(&bin));
}

fn part_one(binary: &str) -> u32 {
    let mut stream = Stream::new(binary);
    let (p, _r) = Packet::new(&mut stream);
    p.sum_versions()
}

fn part_two(binary: &str) -> u64 {
    let mut stream = Stream::new(binary);
    let (p, _r) = Packet::new(&mut stream);
    p.value.unwrap()
}

struct Stream<'a> {
    chars: Chars<'a>,
}

impl<'a> Stream<'a> {
    fn new(binary: &'a str) -> Self {
        Self {
            chars: binary.chars(),
        }
    }

    fn take(&mut self, amount: usize) -> Vec<char> {
        // println!("Taking {}", amount);
        (0..amount).map(|_| self.chars.next().unwrap()).collect()
    }
}

struct Packet {
    version: u32,
    type_id: u32,
    value: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    /// Returns Packet and bytes read
    fn new(stream: &mut Stream) -> (Self, u64) {
        let mut bits_read = 0;
        println!("New packet");
        // println!("   Stream: {:?}", stream.chars);

        // Version
        let bits = stream.take(3).into_iter().collect::<String>();
        let version = u32::from_str_radix(&bits, 2).unwrap();
        bits_read += 3;
        println!("  Version: {}", version);

        // Type
        let bits = stream.take(3).into_iter().collect::<String>();
        let type_id = u32::from_str_radix(&bits, 2).unwrap();
        bits_read += 3;
        println!("  Type: {}", type_id);

        // Literal Value
        if type_id == 4 {
            let mut groups: Vec<String> = Vec::new();
            loop {
                let group = stream.take(5);
                bits_read += 5;

                groups.push(group[1..].into_iter().collect());
                if group[0] == '0' {
                    break;
                }
            }
            let bits = groups.concat();
            let value = u64::from_str_radix(&bits, 2).unwrap();
            println!("  Value: {}", value);
            (
                Self {
                    version,
                    type_id,
                    value: Some(value),
                    sub_packets: None,
                },
                bits_read,
            )
        } else {
            // Operator
            let length_type_id = stream.take(1)[0];
            bits_read += 1;

            let mut sub_packets: Vec<Packet> = Vec::new();
            if length_type_id == '0' {
                // next 15 bits represents total number of bits contained by subpackets
                let bits: String = stream.take(15).into_iter().collect();
                let length = u64::from_str_radix(&bits, 2).unwrap();
                bits_read += 15;

                let mut sub_read_total = 0;
                while sub_read_total < length {
                    println!("{} / {}", sub_read_total, length);
                    let (packet, sub_read) = Packet::new(stream);
                    sub_read_total += sub_read;
                    sub_packets.push(packet);
                }
                bits_read += sub_read_total;
            } else {
                // next 11 bits represent total number of sub_packets immediately contained
                let bits: String = stream.take(11).into_iter().collect();
                let number = u64::from_str_radix(&bits, 2).unwrap();
                bits_read += 11;

                for _ in 0..number {
                    let (sub, read) = Packet::new(stream);
                    sub_packets.push(sub);
                    bits_read += read;
                }
            }
            let value = match type_id {
                0 => {
                    let mut s = 0;
                    for p in &sub_packets {
                        s += match p.value {
                            Some(v) => v,
                            None => continue,
                        };
                    }
                    s
                }
                1 => {
                    let mut s = 1;
                    for p in &sub_packets {
                        s *= match p.value {
                            Some(v) => v,
                            None => continue,
                        };
                    }
                    s
                }
                2 => {
                    let mut s = u64::MAX;
                    for p in &sub_packets {
                        let v = match p.value {
                            Some(v) => v,
                            None => continue,
                        };
                        s = s.min(v);
                    }
                    s
                }
                3 => {
                    let mut s = 0;
                    for p in &sub_packets {
                        let v = match p.value {
                            Some(v) => v,
                            None => continue,
                        };
                        s = s.max(v);
                    }
                    s
                }
                5 => {
                    let a = sub_packets[0].value.unwrap();
                    let b = sub_packets[1].value.unwrap();
                    if a > b {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let a = sub_packets[0].value.unwrap();
                    let b = sub_packets[1].value.unwrap();
                    if a < b {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let a = sub_packets[0].value.unwrap();
                    let b = sub_packets[1].value.unwrap();
                    if a == b {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            };
            (
                Self {
                    version,
                    type_id,
                    value: Some(value),
                    sub_packets: Some(sub_packets),
                },
                bits_read,
            )
        }
    }

    fn sum_versions(&self) -> u32 {
        let mut total = 0;
        total += self.version;
        if let Some(other) = &self.sub_packets {
            for o in other {
                let s = o.sum_versions();
                total += s;
            }
        }
        total
    }
}
