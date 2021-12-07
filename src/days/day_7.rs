use super::super::utils::*;

pub fn day_7() {
    let input = get_input(7);
    // println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let ps: Vec<i32> = input.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let mut v = Vec::new();
    for destination in &ps {
        let mut fuel = 0;
        for p in ps.clone() {
            if p != *destination {
                fuel += (p - destination).abs();
            }
        }
        v.push(fuel);
    }
    *v.iter().min().unwrap()
}

fn part_two(input: &str) -> i32 {
    let ps: Vec<i32> = input.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    let mut v = Vec::new();
    for destination in &ps {
        let mut fuel = 0;
        for p in ps.clone() {
            if p != *destination {
                fuel += calc((p - destination).abs());
            }
        }
        v.push(fuel);
    }
    *v.iter().min().unwrap()
}

fn calc(distance: i32) -> i32 {
    let mut t = 0;
    let mut d = distance;
    while d > 0 {
        t += d;
        d -= 1;
    }
    t
}