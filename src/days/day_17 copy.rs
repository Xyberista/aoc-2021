use std::collections::HashSet;

use super::super::utils::*;

pub fn day_17() {
    let input = get_input(17);
    let input = input.replace("target area: ", "");
    let (a, b) = input.split_once(", ").unwrap();

    // ranges are inclusivee
    let x_range = {
        let i = a.replace("x=", "");
        let (xs, xe) = i.split_once("..").unwrap();
        (
            xs.trim().parse::<i32>().unwrap(),
            xe.trim().parse::<i32>().unwrap(),
        )
    };
    let y_range = {
        let i = b.replace("y=", "");
        let (ys, ye) = i.split_once("..").unwrap();
        (
            ys.trim().parse::<i32>().unwrap(),
            ye.trim().parse::<i32>().unwrap(),
        )
    };
    // println!("{} {}", x_range.0, x_range.1);
    // println!("{} {}", y_range.0, y_range.1);

    println!("Part 1: {}", part_one(&input, x_range, y_range));
    println!("Part 2: {}", part_two(&input, x_range, y_range));
}

fn part_one(input: &str, x_r: (i32, i32), y_r: (i32, i32)) -> i32 {
    let mut best = 0;
    for y in 0..100 {
        for x in 0..100 {
            let mut p = Probe::new(x, y);
            if let Some(v) = p.run(x_r, y_r) {
                best = best.max(v);
            }
        }
    }
    best
}

fn part_two(input: &str, x_r: (i32, i32), y_r: (i32, i32)) -> usize {
    let mut set: Vec<(i32, i32)> = Vec::new();
    for y in -4000..4000 {
        for x in -2000..2000 {
            let mut p = Probe::new(x, y);
            if let Some(v) = p.run(x_r, y_r) {
                set.push((x, y));
            }
        }
    }
    set.len()
}

struct Probe {
    x: i32,
    y: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Probe {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            x_vel,
            y_vel,
        }
    }

    fn step_one(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        let dx = (0 - self.x_vel).signum();
        self.x_vel += dx;
        self.y_vel -= 1;
    }

    fn run(&mut self, (xs, xe): (i32, i32), (ys, ye): (i32, i32)) -> Option<i32> {
        let mut max_y = 0;
        while self.x < xe && self.y > ye {
            self.step_one();
            max_y = max_y.max(self.y);
        }
        if (self.x >= xs && self.x <= xe) && (self.y >= ys && self.y <= ye) {
            Some(max_y)
        } else {
            None
        }
    }
}
