use core::panic;

use super::super::utils::*;

pub fn day_10() {
    let input = get_input(10);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn is_corrupt(line: &str) -> Option<char> {
    let mut line = line.chars();
    let mut del: Vec<char> = Vec::new();
    del.push(line.next().unwrap());
    for c in line {
        if ")]}>".contains(c) {
            let p = del.pop().unwrap();
            if c == ')' {
                if p != '(' {
                    return Some(c);
                }
            } else if c == ']' {
                if p != '[' {
                    return Some(c);
                }
            } else if c == '}' {
                if p != '{' {
                    return Some(c);
                }
            } else if c == '>' {
                if p != '<' {
                    return Some(c);
                }
            }
        } else {
            del.push(c);
        }
    }
    None
}

fn part_one(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        if let Some(c) = is_corrupt(line) {
            score += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            };
        }
    }
    score
}

fn find_missing(line: &str) -> Vec<char> {
    let mut m: Vec<char> = Vec::new();
    let mut line = line.chars();
    m.push(line.next().unwrap());
    for c in line {
        if ")]}>".contains(c) {
            m.pop();
        } else {
            m.push(c);
        }
    }

    let mut n = Vec::new();
    for d in m {
        if d == '(' {
            n.push(')');
        }
        if d == '[' {
            n.push(']');
        }
        if d == '{' {
            n.push('}');
        }
        if d == '<' {
            n.push('>');
        }
    }
    n.reverse();
    n
}

fn part_two(input: &str) -> i64 {
    let mut incomplete = Vec::new();
    let mut scores = Vec::new();
    for line in input.lines() {
        if is_corrupt(line).is_none() {
            incomplete.push(line);
        }
    }

    for line in incomplete {
        let chars = find_missing(line);
        let mut s = 0;
        for c in chars {
            s *= 5;
            s += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }
        scores.push(s);
    }
    scores.sort();
    scores[(scores.len() / 2)]
}
