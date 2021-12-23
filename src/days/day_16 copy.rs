use core::num;
use std::any::type_name;
use std::io::prelude::*;
use std::io::BufReader;
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
    println!("Part 1: {}", part_one(bin.clone()));
    // println!("Part 2: {}", part_two(&input));
}

fn part_one(input: String) -> u64 {
    let mut stream = input.chars();
    let mut read = 0;
    let packet = Packet::new(&mut stream, &mut read);
    sum_versions(packet)
}

fn sum_versions(packet: Packet) -> u64 {
    let mut versions = packet.version;
    if let Some(other) = packet.sub_packets {
        for o in other {
            let s = sum_versions(o);
            versions += s;
        }
    }
    versions
}

struct Packet {
    version: u64,
    type_id: u64,
    value: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    fn new(stream: &mut Chars, bits_read: &mut u64) -> Self {
        // Header
        let v = (0..3).map(|_| stream.next().unwrap()).collect::<String>();
        let version = u64::from_str_radix(&v, 2).unwrap();
        *bits_read += 3;

        let t = (0..3).map(|_| stream.next().unwrap()).collect::<String>();
        let type_id = u64::from_str_radix(&t, 2).unwrap();
        *bits_read += 3;

        println!("Version: {}", version);
        println!("Type: {}", type_id);

        // Body

        // Literal Value packet
        if type_id == 4 {
            let mut value: Vec<String> = Vec::new();
            loop {
                let mut group = (0..5).map(|_| stream.next().unwrap());
                *bits_read += 5;
                let done = group.next().unwrap() == '0';
                let bits = group.collect::<String>();
                value.push(bits);

                if done {
                    break;
                }
            }
            let value = u64::from_str_radix(&value.concat(), 2).unwrap();
            println!("Value: {}", value);
            Self {
                version,
                type_id,
                value: Some(value),
                sub_packets: None,
            }
        } else {
            let length_type_id = stream.next().unwrap();
            *bits_read += 1;

            let mut sub_packets = Vec::new();
            if length_type_id == '0' {
                // next 15 bits represent total length of subpackets in bits
                let bits = (0..15).map(|_| stream.next().unwrap());
                *bits_read += 15;
                let bits = bits.collect::<String>();
                let total_length = u64::from_str_radix(&bits, 2).unwrap();
                println!("total bits of subpackages: {}", total_length);

                let mut read = 0;
                while read < total_length {
                    let packet = Packet::new(stream, &mut read);
                    sub_packets.push(packet);
                }
                println!("### Done Subpackets");
                *bits_read += read;
            } else if length_type_id == '1' {
                // next 11 bits represent number of subpackets
                let bits = (0..11).map(|_| stream.next().unwrap());
                *bits_read += 15;
                let bits = bits.collect::<String>();
                let number_of_subpackets = u64::from_str_radix(&bits, 2).unwrap();
                println!("Total number of subpackages: {}", number_of_subpackets);
                let mut read = 0;
                for _ in 0..number_of_subpackets {
                    let packet = Packet::new(stream, &mut read);
                    sub_packets.push(packet);
                }
                println!("### Done subpackets");
                *bits_read += read;
            }

            Self {
                version,
                type_id,
                value: None,
                sub_packets: Some(sub_packets),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        let h = "F";
        let b = to_binary(h);
        assert_eq!(&b, "1111");
    }

    #[test]
    fn test_version_sum_1() {
        let bin = to_binary("8A004A801A8002F478");
        let mut bin = bin.chars();
        let p = Packet::new(&mut bin, &mut 0);
        assert_eq!(sum_versions(p), 16);
    }

    #[test]
    fn test_version_sum_2() {
        let bin = to_binary("620080001611562C8802118E34");
        let mut bin = bin.chars();
        let p = Packet::new(&mut bin, &mut 0);
        assert_eq!(sum_versions(p), 12);
    }

    #[test]
    fn test_version_sum_3() {
        let bin = to_binary("C0015000016115A2E0802F182340");
        let mut bin = bin.chars();
        let p = Packet::new(&mut bin, &mut 0);
        assert_eq!(sum_versions(p), 23);
    }

    #[test]
    fn test_version_sum_4() {
        let bin = to_binary("A0016C880162017C3686B18A3D4780");
        let mut bin = bin.chars();
        let p = Packet::new(&mut bin, &mut 0);
        assert_eq!(sum_versions(p), 31);
    }
}
