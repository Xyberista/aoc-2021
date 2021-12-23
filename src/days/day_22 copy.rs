use std::collections::{HashMap, HashSet};

use super::super::utils::*;

type Point = (i32, i32);

enum State {
    On(Point, Point, Point),
    Off(Point, Point, Point),
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
    // println!("Part 1: {}", part_one(&commands));
    println!("Part 2: {}", part_two(&commands));
}

fn part_one(commands: &[State]) -> usize {
    let mut grid: HashSet<(i32, i32, i32)> = HashSet::new();
    for cmd in commands {
        match *cmd {
            State::On(x, y, z) => {
                if (x.0 >= -50 && x.1 <= 50)
                    && (y.0 >= -50 && y.1 <= 50)
                    && (z.0 >= -50 && z.1 <= 50)
                {
                    for xp in x.0..=x.1 {
                        for yp in y.0..=y.1 {
                            for zp in z.0..=z.1 {
                                grid.insert((xp, yp, zp));
                            }
                        }
                    }
                }
            }
            State::Off(x, y, z) => {
                if (x.0 >= -50 && x.1 <= 50)
                    && (y.0 >= -50 && y.1 <= 50)
                    && (z.0 >= -50 && z.1 <= 50)
                {
                    for xp in x.0..=x.1 {
                        for yp in y.0..=y.1 {
                            for zp in z.0..=z.1 {
                                grid.remove(&(xp, yp, zp));
                            }
                        }
                    }
                }
            }
        }
    }
    grid.len()
}

type Cuboid = ((i32, i32), (i32, i32), (i32, i32));

fn intersecting_points(&(x,y,z): &Cuboid, &(ox, oy, oz): &Cuboid) -> Option<Cuboid> {
    let mut xs = 0;
    let mut xe = 0;
    let mut ys = 0;
    let mut ye = 0;
    let mut zs = 0;
    let mut ze = 0;
    // x borders
    if x.0 > ox.1 {
        // no intersection
        return None;
    } else if x.0 >= ox.0 {
        xs = x.0;
    } else if ox.0 <= x.1 {
        xs = ox.0
    }
    if x.1 < ox.0 {
        // no intersection
        return None;
    } else if ox.1 >= x.0 {
        xe = ox.1;
    } else if x.1 >= ox.0 {
        xe = x.1
    }

    // y borders
    if y.0 > oy.1 {
        // no intersection
        return None;
    } else if y.0 >= oy.0 {
        ys = y.0;
    } else if oy.0 <= y.1 {
        ys = oy.0
    }
    if y.1 < oy.0 {
        // no intersection
        return None;
    } else if oy.1 >= y.0 {
        ye = oy.1;
    } else if y.1 >= oy.0 {
        ye = y.1
    }

    // z borders
    if z.0 > oz.1 {
        // no intersection
        return None;
    } else if z.0 >= oz.0 {
        zs = z.0;
    } else if oz.0 <= z.1 {
        zs = oz.0
    }
    if z.1 < oz.0 {
        // no intersection
        return None;
    } else if oz.1 >= z.0 {
        ze = oz.1;
    } else if y.1 >= oz.0 {
        ze = z.1
    }

    // definitely intersecting
    // println!("{}, {}", xs, xe);
    // println!("{}, {}", ys, ye);
    // println!("{}, {}", zs, ze);
    Some(((xs,xe),(ys,ye),(zs,ze)))
}

fn cuboid_volume((x,y,z): &Cuboid) -> i64 {
    ((x.1 + 1 - x.0).abs() * (y.1 + 1 - y.0).abs() * (z.1 + 1 - z.0).abs()) as i64
}

fn part_two(commands: &[State]) -> i64 {
    let mut on: HashSet<((i32, i32), (i32, i32), (i32, i32))> = HashSet::new();
    let mut off: HashSet<Cuboid> = HashSet::new();
    let mut volume: i64 = 0;
    for cmd in commands {
        match *cmd {
            State::On(x, y, z) => {
                let mut new = HashSet::new();
                let one = (x,y,z);
                println!("Adding {:?}", one);
                for two @ (x1, y1, z1) in on {
                    if let Some(intersect @ (ix, iy, iz)) = intersecting_points(&one, &two) {
                        println!("INTERSECT");
                        println!("  Cuboid 1: {:?}", one);
                        println!("  Cuboid 2: {:?}", two);
                        println!("  intersect: {:?}", intersect);
                        let a = ((ix.0.min(x.0), x.1.max(ix.1)), (iy.0.min(y.0), y.1.max(iy.1)), (iz.1.max(z.0),z.1.max(iz.0)));
                        println!("  A: {:?}", a);
                    } 
                    new.insert(two);
                }
                if new.is_empty() {
                    new.insert(one);
                }
                on = new;
            }
            State::Off(x, y, z) => {
                
            }
        }
    }
    println!("{:?}", on);
    for c in on {
        volume += cuboid_volume(&c);
    }
    volume
}
