use std::collections::HashMap;

use super::super::utils::*;

pub fn day_8() {
    let input = get_input(8);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut t = 0;
    for line in input.lines() {
        let (pat, out) = line.split_once("|").unwrap();
        let mut n: HashMap<i32, String> = HashMap::new();
        let pat = pat.trim();
        let out = out.trim();
        for p in pat.split(" ").chain(out.split(" ")) {
            let mut p = p.chars().collect::<Vec<char>>();
            p.sort();
            let p = p.into_iter().collect::<String>();
            if p.len() == 2 {
                n.insert(1, p);
            } else if p.len() == 4 {
                n.insert(4, p);
            } else if p.len() == 3 {
                n.insert(7, p);
            } else if p.len() == 7 {
                n.insert(8, p);
            }
        }
        let v = n.values().collect::<Vec<_>>();
        let mut c = 0;
        for p in out.split(" ") {
            let mut p = p.chars().collect::<Vec<char>>();
            p.sort();
            let p = p.into_iter().collect::<String>();
            if v.contains(&&p) {
                c += 1;
            }
        }
        t += c;
    }
    t
}

fn part_two(input: &str) -> i32 {
    let mut t = 0;
    for line in input.lines() {
        let (pre, out) = line.split_once(" | ").unwrap();
       let map = parse_row(pre);
       let digits: String = out.split(" ").map(|s| to_digit(&map, s)).collect();
       let number = digits.parse::<i32>().unwrap();
       t += number;
    }
    t
}

/// Returns sorted segments grouped by length
fn to_map(row: &str) -> HashMap<i32, Vec<String>> {
    let mut m: HashMap<i32, Vec<String>> = HashMap::new();
    for i in row.replace("|", "").split(" ") {
        let mut c = i.chars().collect::<Vec<_>>();
        c.sort();
        let c: String = c.into_iter().collect();
        let len = c.len() as i32;
        m.entry(len).or_default().push(c);
    }
    m
}

fn to_digit(map: &HashMap<String, i32>, input: &str) -> char {
    let mut s: Vec<char> = input.chars().collect();
    s.sort();
    let s: String = s.into_iter().collect();
    map[&s].to_string().chars().next().unwrap()
}

fn parse_row(row: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    let lens = to_map(row);

    let one = lens[&2][0].clone();
    let four = lens[&4][0].clone();
    let seven = lens[&3][0].clone();
    let eight = lens[&7][0].clone();

    let a: char = seven.chars().filter(|c| !one.contains(*c)).next().unwrap();
    let bd: String = four.chars().filter(|c| !one.contains(*c)).collect();
    let eg: String = eight.chars().filter(|c| !seven.contains(*c) && !bd.contains(*c)).collect();

    let mut two: String = String::new();
    let mut three: String = String::new();
    let mut five: String = String::new();

    for f in &lens[&5] {
        if bd.chars().all(|c| f.contains(c)) {
            five = f.to_owned();
        } else if eg.chars().all(|c| f.contains(c)) {
            two = f.to_owned();
        } else {
            three = f.to_owned();
        }
    }

    let mut zero: String = String::new();
    let mut six: String = String::new();
    let mut nine: String = String::new();
    
    for s in &lens[&6] {
        if !one.chars().all(|c| s.contains(c)) {
            six = s.to_owned();
        } else if !bd.chars().all(|c| s.contains(c)) {
            zero = s.to_owned();
        } else {
            nine = s.to_owned();
        }
    }

    map.insert(zero, 0);
    map.insert(one, 1);
    map.insert(two, 2);
    map.insert(three, 3);
    map.insert(four, 4);
    map.insert(five, 5);
    map.insert(six, 6);
    map.insert(seven, 7);
    map.insert(eight, 8);
    map.insert(nine, 9);
    map
}
