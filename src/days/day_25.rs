use std::collections::{HashMap, BTreeMap};

use super::super::utils::*;

type Position = (usize, usize);
type Map = BTreeMap<Position, char>;

pub fn day_25() {
    let input = get_input(25);
    let mut map: BTreeMap<Position, char> = BTreeMap::new();
    let mut my = 0;
    let mut mx = 0;
    for (y, line) in input.lines().enumerate() {
        my += 1;
        mx = line.len();
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert((x,y), c);
            }
        }
    }
    // println!("{:?}", map);
    println!("Part 1: {}", part_one(&map, mx, my));
    // println!("Part 2: {}", part_two(&input));
}

fn step_one(map: &Map, mx: usize, my: usize) -> Option<Map> {
    let mut a: Map = BTreeMap::new();
    // horiz
    for ((x,y), c) in map {
        if *c == '>' {
            let nx = if *x + 1 == mx { 0 } else { *x + 1 };
            if map.contains_key(&(nx, *y)) {
                a.insert((*x,*y), *c);
            } else {
                a.insert((nx,*y), *c);
            }
        } else {
            a.insert((*x, *y), *c);
        }
    }
    // vert
    let mut b: Map = BTreeMap::new();
    for ((x,y), c) in &a {
        if *c == 'v' {
            let ny = if *y + 1 == my { 0 } else { *y + 1 };
            if a.contains_key(&(*x, ny)) {
                b.insert((*x,*y), *c);
            } else {
                b.insert((*x,ny), *c);
            }
        } else {
            b.insert((*x, *y), *c);
        }
    }
    if b == *map {
        None
    } else {
        Some(b)
    }
}

fn part_one(map: &Map, mx: usize, my: usize) -> i32 {
    let mut t = 1;
    let mut map = map.clone();
    while let Some(new) = step_one(&map, mx, my) {
        map = new;
        t += 1;
    }
    t
}