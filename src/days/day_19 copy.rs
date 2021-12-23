use std::collections::{HashMap, BTreeSet};

use super::super::utils::*;

type Point = (i32, i32, i32);
type Scanners = HashMap<usize, Vec<Point>>;

pub fn day_19() {
    let input = get_input(19);
    let mut scanners = HashMap::new();
    for (i, scan) in input.split("\n\n").enumerate() {
        let mut points: Vec<Point> = Vec::new();
        let lines = scan.lines().collect::<Vec<_>>();
        for &line in lines[1..].into_iter() {
            let s = line.trim().split(",").collect::<Vec<_>>();
            let a = s[0].parse::<i32>().unwrap();
            let b = s[1].parse::<i32>().unwrap();
            let c = s[2].parse::<i32>().unwrap();
            let point = (a,b,c);
            points.push(point);
        }
        scanners.insert(i, points);
    }
    // println!("{:?}", scanners);
    println!("Part 1: {}", part_one(scanners));
    // println!("Part 2: {}", part_two(&input));
}

fn distance(a: &Point, b: &Point) -> f32 {
    let pa = (b.0 - a.0).pow(2);
    let pb = (b.1 - a.1).pow(2);
    let pc = (b.2 - a.2).pow(2);
    ((pa + pb + pc) as f32).sqrt()
}

fn find_pairs(sc_a: &Vec<Point>, sc_b: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    let mut both_a = BTreeSet::new();
    let mut both_b = BTreeSet::new();
    for aa in sc_a {
        for ab in sc_a {
            if aa == ab {
                continue;
            }
            for ba in sc_b {
                for bb in sc_b {
                    if ba == bb {
                        continue;
                    }
                    let off_a = distance(aa, ab);
                    let off_b = distance(ba, bb);
                    // println!("{:?} {:?}", aa, ab);
                    // println!("{:?} {:?}", ba, bb);
                    // println!();
                    if off_a == off_b {
                        both_a.insert(*aa);
                        both_a.insert(*ab);
                        both_b.insert(*ba);
                        both_b.insert(*bb);
                    }
                }
            }
        }
    }
    // (both_a.into_iter().collect::<Vec<_>>(), both_b.into_iter().collect::<Vec<_>>())
    let mut both_a = both_a.into_iter().collect::<Vec<_>>();
    let mut both_b = both_b.into_iter().collect::<Vec<_>>();
    both_a.sort();
    both_b.sort();
    (both_a, both_b)
}

fn get_signs(p: &Point) -> (i32, i32, i32) {
    let a = (p.0).signum();
    let b = (p.1).signum();
    let c = (p.2).signum();
    (a, b, c)
}

// sees if corrent orientation
fn compare_two(a: &Vec<Point>, b: &Vec<Point>) -> Option<(i32, i32, i32)> {
    // a is at relative 0,0,0
    let (pa, pb) = find_pairs(a, b);
    // println!("{:?}\n{:?}", pa, pb);
    if pa.len() >= 12 && pb.len() >= 12 {
        // 12 matching, returned pairs should be in the same order.
        let mut pan = Vec::new();
        for i in &pa {
            for j in &pa {
                if i != j {
                    pan.push((i, j));
                }
            }
        }
        // let pbn = pb.iter().zip(pb[1..].iter());
        let mut pbn = Vec::new();
        for i in &pb {
            for j in &pb {
                if i != j {
                    pbn.push((i, j));
                }
            }
        }

        let mut dx = 0;
        let mut dy = 0;
        let mut dz = 0;

        for (a, b) in pan.iter().zip(pbn) {
            println!("{} {} {}", dx, dy, dz);
            if distance(a.0, a.1) == distance(b.0, b.1) {
                // println!("{:?} {:?}", a, b);
                let a = a.0;
                let b = b.0;
                let ndx = a.0 - b.0;
                if ndx != dx && dx != 0 {
                    println!("B-x {} {}", dx, ndx);
                    return None;
                } else {
                    dx = ndx;
                }
                let ndy = a.1 - b.1;
                if ndy != dy && dy != 0 {
                    println!("B-y {} {}", dy, ndy);
                    return None;
                } else {
                    dy = ndy;
                }
                let ndz = a.2 - b.2;
                if ndz != dz && dz != 0 {
                    println!("B-z {} {}", dz, ndz);
                    return None;
                } else {
                    dz = ndz;
                }
                // println!("{} {} {}", dx, dy, dz);
            } else {
                return None
            }
        }
        println!("Correct orientation :D");
        Some((dx, dy, dz))
    } else {
        None
    }
}

/// Signs_a and Signs_b as input
fn compare_signs(a: &Point, b: &Point) -> bool {
    for x in [1,-1] {
        for y in [1,-1] {
            for z in [1,-1] {
                let c = (b.0 * x, b.1 * y, b.2 * z);
                if (a.0 == c.0 || c.0 == 0)
                    && (a.1 == c.1 || c.1 == 0)
                    && (a.2 == c.2 || c.2 == 0) {
                    return true;
                }
            }
        }
    }
    false
}

fn orient((x, y, z, shift): (i32, i32, i32, usize), p: &Point) -> Point {
    let mut new = [p.0 * x, p.1 * y, p.2 * z];
    new.rotate_left(shift);
    let a = new[0];
    let b = new[1];
    let c = new[2];
    (a,b,c)
}

fn orient_scanner(orientation: (i32, i32, i32, usize), sc: &mut Vec<Point>) {
    *sc = sc.into_iter().map(|p| orient(orientation, p)).collect();
}

/// Correct orientation of b relative to a, returns paired a and b
fn correct_orientation(a: &Vec<Point>, b: &mut Vec<Point>) -> Vec<(Point, Point)> {
    let p = [1,-1];
    for shift in 0..3 {
        for x in p {
            for y in p {
                for z in p {
                    let orientation = (x,y,z,shift);
                    orient_scanner(orientation, b);
                    println!("or {} {} {} {}", x, y, z, shift);
                    if let Some(offset) = compare_two(a, &b) {
                        println!("Offset: {:?}", offset);
                        return Vec::new();
                    }
                }
            }
        }
    }
    unimplemented!()
}

fn part_one(scanners: Scanners) -> i32 {
    let mut a = scanners[&0].clone();
    let mut b = scanners[&1].clone();
    let paired = correct_orientation(&a, &mut b);
    println!("{}", paired.len());
    // println!("{:?}\n{:?}\n", a, b);
    // let mut sa = a.clone();
    // sa.sort();
    // b.sort();
    // println!("{:?}\n{:?}", sa, b);

    println!("Paired: {:?}\n", paired);
    a.sort();
    b.sort();
    println!("{:?}", a);
    println!("{:?}", b);
    0
}