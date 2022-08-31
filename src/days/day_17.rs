use std::ops::RangeInclusive;

use super::super::utils::*;

pub fn day_17() {
    let input = get_input(17);
    let input = input.replace("target area: ", "");
    let (a, b) = input.split_once(", ").unwrap();

    // ranges are inclusivee
    let x_range = {
        let i = a.replace("x=", "");
        let (xs, xe) = i.split_once("..").unwrap();
        let xs = xs.trim().parse::<i32>().unwrap();
        let xe = xe.trim().parse::<i32>().unwrap();
        (xs, xe)
    };
    let y_range = {
        let i = b.replace("y=", "");
        let (ys, ye) = i.split_once("..").unwrap();
        let ys = ys.trim().parse::<i32>().unwrap();
        let ye = ye.trim().parse::<i32>().unwrap();
        (ys, ye)
    };

    println!("Part 1: {}", part_one(&input, x_range, y_range));
    println!("Part 2: {}", part_two(&input, x_range, y_range));
}

type P = (i32, i32);
type Range = RangeInclusive<i32>;

fn get_heights((xs, xe): P, (ys, ye): P) -> Vec<i32> {
    let mut heights = Vec::new();
    for y in -500..500 {
        for x in -500..500 {
            if let Some(height) = run((x, y), (xs, xe), (ys, ye)) {
                heights.push(height);
            }
        }
    }
    heights
}

fn part_one(_input: &str, (xs, xe): P, (ys, ye): P) -> i32 {
    let mut heights = Vec::new();
    for y in -500..500 {
        for x in -500..500 {
            let r = run((x, y), (xs, xe), (ys, ye));
            if let Some(height) = r {
                heights.push(height);
            }
        }
    }
    heights.into_iter().max().unwrap()
}

fn part_two(_input: &str, (xs, xe): P, (ys, ye): P) -> usize {
    let mut heights = Vec::new();
    for y in -500..500 {
        for x in -500..500 {
            let r = run((x, y), (xs, xe), (ys, ye));
            if let Some(height) = r {
                heights.push(height);
            }
        }
    }
    heights.len()
}

fn run(v: P, (xs, xe): P, (ys, ye): P) -> Option<i32> {
    let mut p: P = (0, 0);
    let mut v = v;
    let x_range = xs..=xe;
    let y_range = ys..=ye;
    let mut heights = Vec::new();
    loop {
        p.0 += v.0;
        p.1 += v.1;
        v.0 += (-v.0).signum();
        v.1 -= 1;

        heights.push(p.1);

        if v.0 == 0 && (!x_range.contains(&p.0) || p.1 < ys) {
            return None;
        }
        if x_range.contains(&p.0) && y_range.contains(&p.1) {
            let max = heights.into_iter().max().unwrap();
            return Some(max);
        }
    }
}
