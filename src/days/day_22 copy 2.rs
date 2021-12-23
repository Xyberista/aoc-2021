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
        let xe = x.next().unwrap().parse::<i32>().unwrap();
        let y = s.next().unwrap().replace("y=", "");
        let mut y = y.split("..");
        let ys = y.next().unwrap().parse::<i32>().unwrap();
        let ye = y.next().unwrap().parse::<i32>().unwrap();
        let z = s.next().unwrap().replace("z=", "");
        let mut z = z.split("..");
        let zs = z.next().unwrap().parse::<i32>().unwrap();
        let ze = z.next().unwrap().parse::<i32>().unwrap();
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
    test();
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

fn split_cuboid((x,y,z): &Cuboid, (ix,iy,iz): &Cuboid) -> Vec<Cuboid> {
    let mut v: Vec<Cuboid> = Vec::new();
    //left
    v.push(((x.0,ix.0),(y.0,y.1),(z.0,z.1)));
    // right
    v.push(((ix.1,x.1),(y.0,y.1),(z.0,z.1)));
    // middle
    //  behind
    v.push(((ix.0,ix.1),(iy.1,y.1),(z.0,z.1)));
    //  front
    v.push(((ix.0,ix.1),(y.0,iy.0),(z.0,z.1)));
    //  top
    v.push(((ix.0,ix.1),(y.0,iy.1),(iz.1,z.1)));
    //  bottom
    v.push(((ix.0,ix.1),(y.0,iy.0),(z.0,iz.0)));
    let v = v.into_iter().filter(|(x,y,z)| {
        x.0 != x.1 && y.0 != y.1 && z.0 != z.1
    }).collect();
    println!("{:?}", v);
    v
}

fn cuboid_volume((x,y,z): &Cuboid) -> i64 {
    ((x.1 + 1 - x.0).abs() * (y.1 + 1 - y.0).abs() * (z.1 + 1 - z.0).abs()) as i64
}

fn part_one(commands: &[State]) -> i64 {
    let mut grid: HashSet<Cuboid> = HashSet::new();
    for cmd in commands {
        match cmd {
            State::On(x, y, z) => {
                let c = (*x,*y,*z);
                let mut g = grid.clone();
                let mut f = false;
                for two in grid {
                    if let Some(intersect) = intersection_cuboid(&c, &two) {
                        let s = split_cuboid(&two, &intersect);
                        g.extend(s.into_iter());
                        f = true;
                    }
                }
                if !f {
                    g.insert(c);
                }
                grid = g;
            }
            State::Off(x, y, z) => {

            }
        }
    }
    let mut v = 0;
    for c in grid {
        let vol = cuboid_volume(&c);
        v += vol;
    }
    v
}

fn test() {
    let a = ((10,12),(10,12),(10,12));
    println!("{}", cuboid_volume(&a));
    let b = ((11,13),(11,13),(11,13));
    println!("{}", cuboid_volume(&b));
    if let Some(intersect) = intersection_cuboid(&a, &b) {
        println!("{:?}", intersect);
    }
}