use std::collections::BTreeMap;

use super::super::utils::*;

pub fn day_5() {
    let input = get_input(5);
    println!("Part 1: {}", calc(&input, false));
    println!("Part 2: {}", calc(&input, true));
}

fn parse_row(row: &str) -> [i32; 4] {
    let a: Vec<i32> = row
        .split(" -> ")
        .flat_map(|s| s.split(','))
        .map(|i| i.parse().unwrap())
        .collect();
    let xs = a[0];
    let ys = a[1];
    let xe = a[2];
    let ye = a[3];
    [xs, ys, xe, ye]
}

fn calc(input: &str, check_diagonals: bool) -> i32 {
    let mut board: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    for line in input.lines() {
        let [xs, ys, xe, ye] = parse_row(line);
        let dx = (xe - xs).signum();
        let dy = (ye - ys).signum();
        if !check_diagonals {
            if dx != 0 && dy != 0 {
                continue;
            }
        }
        let mut x = xs;
        let mut y = ys;
        *board.entry((x, y)).or_default() += 1;
        while x != xe || y != ye {
            x += dx;
            y += dy;
            *board.entry((x, y)).or_default() += 1;
        }
    }
    let mut t = 0;
    for i in board.values() {
        if *i > 1 {
            t += 1;
        }
    }
    t
}
