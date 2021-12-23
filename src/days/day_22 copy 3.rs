use std::collections::{HashMap, HashSet};

use super::super::utils::*;

type Edge = (i32, i32);

enum State {
    On(Edge, Edge, Edge),
    Off(Edge, Edge, Edge),
}

pub fn day_22() {
    let input = get_input(22);
    let mut commands = Vec::new();
    for line in input.lines() {
        let (st, rest) = line.split_once(" ").unwrap();
        let mut s = rest.split(",");
        let x = s.next().unwrap().replace("x=", "");
        let mut x = x.split("..");
        let xs = x.next().unwrap().parse::<i32>().unwrap();
        let xe = x.next().unwrap().parse::<i32>().unwrap() + 1;
        let y = s.next().unwrap().replace("y=", "");
        let mut y = y.split("..");
        let ys = y.next().unwrap().parse::<i32>().unwrap();
        let ye = y.next().unwrap().parse::<i32>().unwrap() + 1;
        let z = s.next().unwrap().replace("z=", "");
        let mut z = z.split("..");
        let zs = z.next().unwrap().parse::<i32>().unwrap();
        let ze = z.next().unwrap().parse::<i32>().unwrap() + 1;
        let x = (xs, xe);
        let y = (ys, ye);
        let z = (zs, ze);
        let st = match st {
            "on" => State::On(x, y, z),
            "off" => State::Off(x, y, z),
            _ => unreachable!(),
        };
        commands.push(st);
    }
    // test();
    println!("Part 1: {}", part_one(&commands));
    // println!("Part 2: {}", part_two(&commands));
}

type Cuboid = ((i32,i32),(i32,i32),(i32,i32));

fn intersection_cuboid(&(x,y,z): &Cuboid, &(ox, oy, oz): &Cuboid) -> Option<Cuboid> {
    if (x.0 > ox.1 || x.1 < ox.0) || (y.0 > oy.1 || y.1 < ox.0) || (z.0 > oz.1 || z.1 < oz.0) {
        return None;
    }
    
    // x borders
    let xs = x.0.max(ox.0);
    let xe = x.1.min(ox.1);

    // y borders
    let ys = y.0.max(oy.0);
    let ye = y.1.min(oy.1);

    // z borders
    let zs = z.0.max(oz.0);
    let ze = z.1.min(oz.1);

    Some(((xs,xe),(ys,ye),(zs,ze)))
}

fn cuboid_volume((x,y,z): &Cuboid) -> i64 {
    let x = (x.1 - x.0).abs();
    let y = (y.1 - y.0).abs();
    let z = (z.1 - z.0).abs();
    x as i64 * y as i64 * z as i64
}

fn part_one(commands: &[State]) -> i64 {
    let mut on: HashSet<Cuboid> = HashSet::new();
    let mut volume = 0;
    let mut intersections: HashSet<Cuboid> = HashSet::new();
    for cmd in commands {
        match cmd {
            State::On(x, y, z) => {
                if x.0 < -50 || x.1 > 50 || y.0 < -50 || y.1 > 50 || z.0 < -50 || z.1 > 50 {
                    continue;
                }
                let mut new = HashSet::new();
                let c = (*x,*y,*z);
                let mut vol = cuboid_volume(&c);
                for o in &on {
                    if let Some(intersect) = intersection_cuboid(&c, o) {
                        intersections.insert(intersect);
                        let i_vol = cuboid_volume(&intersect);
                        vol -= i_vol;
                    }
                    new.insert(*o);
                }
                volume += vol;
                new.insert(c);
                on = new;
            }
            State::Off(x, y, z) => {
                if x.0 < -50 || x.1 > 50 || y.0 < -50 || y.1 > 50 || z.0 < -50 || z.1 > 50 {
                    continue;
                }
                let c = (*x,*y,*z);
                let mut remove_volume = 0;
                for o in on.iter() {
                    if let Some(intersect) = intersection_cuboid(&c, o) {
                        let i_vol = cuboid_volume(&intersect);
                        // println!("{}", i_vol);
                        remove_volume += i_vol;
                    }
                }
                let mut new = HashSet::new();
                for i in intersections.iter() {
                    if let Some(intersect) = intersection_cuboid(&c, i) {
                        let i_vol = cuboid_volume(&intersect);
                        remove_volume -= i_vol;
                    } else {
                        new.insert(*i);
                    }
                }
                intersections = new;
                volume -= remove_volume;
            }
        }
    }
    println!("{:?}", on);
    volume
}

fn part_two(commands: &[State]) -> i64 {
    let mut on: HashSet<Cuboid> = HashSet::new();
    let mut volume = 0;
    let mut intersections: HashSet<Cuboid> = HashSet::new();
    for cmd in commands {
        match cmd {
            State::On(x, y, z) => {
                let mut new = HashSet::new();
                let c = (*x,*y,*z);
                let mut vol = cuboid_volume(&c);
                for o in &on {
                    if let Some(intersect) = intersection_cuboid(&c, o) {
                        intersections.insert(intersect);
                        let i_vol = cuboid_volume(&intersect);
                        vol -= i_vol;
                    }
                    new.insert(*o);
                }
                volume += vol;
                new.insert(c);
                on = new;
            }
            State::Off(x, y, z) => {
                let c = (*x,*y,*z);
                let mut remove_volume = 0;
                for o in on.iter() {
                    if let Some(intersect) = intersection_cuboid(&c, o) {
                        let i_vol = cuboid_volume(&intersect);
                        // println!("{}", i_vol);
                        remove_volume += i_vol;
                    }
                }
                let mut new = HashSet::new();
                for i in intersections.iter() {
                    if let Some(intersect) = intersection_cuboid(&c, i) {
                        let i_vol = cuboid_volume(&intersect);
                        remove_volume -= i_vol;
                    } else {
                        new.insert(*i);
                    }
                }
                intersections = new;
                volume -= remove_volume;
            }
        }
    }
    println!("{:?}", on);
    volume
}
