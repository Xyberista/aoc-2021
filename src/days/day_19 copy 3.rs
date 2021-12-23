use std::collections::{BTreeSet, HashMap, HashSet};

use super::super::utils::*;

type Point = (i64, i64, i64);
type Scanners = HashMap<usize, Vec<Point>>;

pub fn day_19() {
    let input = get_input(19);
    let mut scanners = HashMap::new();
    for (i, scan) in input.split("\n\n").enumerate() {
        let mut points: Vec<Point> = Vec::new();
        let lines = scan.lines().collect::<Vec<_>>();
        for &line in lines[1..].into_iter() {
            let s = line.trim().split(",").collect::<Vec<_>>();
            let a = s[0].parse::<i64>().unwrap();
            let b = s[1].parse::<i64>().unwrap();
            let c = s[2].parse::<i64>().unwrap();
            let point = (a, b, c);
            points.push(point);
        }
        scanners.insert(i, points);
    }
    // println!("{:?}", scanners);
    println!("Part 1: {}", part_one(scanners));
    // test(scanners);
    // println!("Part 2: {}", part_two(&input));
}

fn distance(a: &Point, b: &Point) -> i64 {
    let pa = (b.0 - a.0).pow(2);
    let pb = (b.1 - a.1).pow(2);
    let pc = (b.2 - a.2).pow(2);
    pa + pb + pc
}

fn orient(shift: usize, p: &Point) -> Point {
    let x = p.0;
    let y = p.1;
    let z = p.2;
    match shift {
        // x @ x
        0 => (x, y, z),
        1 => (x, -z, y),
        2 => (x, -y, -z),
        3 => (x, z, -y),
        // x @ -x
        4 => (-x, -y, z),
        5 => (-x, -z, -y),
        6 => (-x, y, -z),
        7 => (-x, z, y),
        // x @ y
        8 => (-y, x, z),
        9 => (-z, x, -y),
        10 => (y, x, -z),
        11 => (z, x, y),
        // x @ -y
        12 => (-z, -x, y),
        13 => (-y, -x, -z),
        14 => (z, -x, -y),
        15 => (y, -x, z),
        // x @ z
        16 => (-z, y, x),
        17 => (-y, -z, x),
        18 => (z, -y, x),
        19 => (y, z, x),
        // x @ -z
        20 => (z, y, -x),
        21 => (-y, z, -x),
        22 => (-z, -y, -x),
        23 => (y, -z, -x),
        _ => unimplemented!()
    }
}

fn orient_scanner(shift: usize, sc: &mut Vec<Point>) {
    *sc = sc.into_iter().map(|p| orient(shift, p)).collect();
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

fn compare_two(a: &Vec<Point>, b: &Vec<Point>) -> Option<(i64, i64, i64)> {
    let (pa, pb) = find_pairs(a, b);
    if pa.len() >= 12 && pb.len() >= 12 {
        let mut pan = Vec::new();
        for i in &pa {
            for j in &pa {
                if i != j {
                    pan.push((i, j));
                }
            }
        }
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

        for (a, b) in pan.iter().zip(pbn.iter()) {
            if distance(a.0, a.1) == distance(b.0, b.1) {
                let a = a.0;
                let b = b.0;

                let ndx = a.0 - b.0;
                if ndx != dx && dx != 0 {
                    return None;
                } else {
                    dx = ndx;
                }
                let ndy = a.1 - b.1;
                if ndy != dy && dy != 0 {
                    return None;
                } else {
                    dy = ndy;
                }
                let ndz = a.2 - b.2;
                if ndz != dz && dz != 0 {
                    return None;
                } else {
                    dz = ndz;
                }
            } else {
                return None;
            }
        }

        Some((dx, dy, dz))
    } else {
        None
    }
}

/// Returns (orientation, location of scanner)
fn find_orientation(
    a: &Vec<Point>,
    b: &Vec<Point>,
) -> Option<(usize, (i64, i64, i64))> {
    for shift in 0..24 {
        let mut new_b = b.clone();
        orient_scanner(shift, &mut new_b);
        if let Some(location) = compare_two(a, &new_b) {
            return Some((shift, location));
        }
    }
    None
}

fn part_one(scanners: Scanners) -> usize {
    let mut scanners = scanners;
    let mut oriented: Vec<usize> = vec![0];
    let mut locations: HashMap<usize, (i64, i64, i64)> = HashMap::new();
    let mut unvisited: Vec<usize> = (0..scanners.len()).collect();
    let mut known: HashSet<Point> = scanners[&0].iter().cloned().collect();
    locations.insert(0, (0, 0, 0));
    loop {
        eprintln!("{:?}", unvisited);
        // println!("{:?}, known: {}", unvisited, known.len());
        // println!("  {:?}", locations);
        if unvisited.len() == 0 {
            break;
        }
        let i = unvisited.remove(0);
        let mut success = false;
        for k in oriented.iter() {
            // println!("{} {}", i, k);
            let a = scanners[k].clone();
            let mut o = scanners[&i].clone();
            // found overlap, orient and set absolute position
            if let Some((shift, location)) = find_orientation(&a, &o) {
                locations.insert(i, location);

                orient_scanner(shift, &mut o);
                oriented.push(i);

                let o = o
                    .into_iter()
                    .map(|p| (location.0 + p.0, location.1 + p.1, location.2 + p.2))
                    .collect::<Vec<_>>();
                // println!("--- {:?}", o);
                known.extend(o.iter());
                scanners.insert(i, o);
                success = true;
                break;
            }
        }
        if !success {
            unvisited.push(i);
        }
    }

    known.len()
}
