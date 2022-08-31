use super::super::utils::*;

use std::fmt;

pub fn day_18() {
    let input = get_input(18);
    let input = input.trim();
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) -> i32 {
    let mut numbers = input.lines();
    let mut n = Num::new(numbers.next().unwrap());
    for other in numbers {
        let o = Num::new(other);
        n = n.add(o);
        n.reduce();
    }
    println!("Final: {}", n);
    let magnitude = n.magnitude();
    println!("Magnutide: {}", magnitude);
    magnitude
}

fn part_two(input: &str) -> i32 {
    let mut best = 0;
    let numbers = input.lines().map(Num::new).collect::<Vec<Num>>();
    for a in numbers.iter() {
        for b in numbers.iter() {
            let a = a.clone();
            let b = b.clone();
            let mut c = a.add(b);
            c.reduce();
            let m = c.magnitude();
            best = best.max(m);
        }
    }
    println!("Best: {}", best);
    best
}

#[derive(Debug, Clone)]
enum Num {
    Val(i32),
    Pair { left: Box<Num>, right: Box<Num> },
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Val(v) => write!(f, "{}", v),
            Num::Pair { left, right } => {
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}

impl Num {
    fn new(input: &str) -> Self {
        if input.contains('[') {
            let input = input.chars().collect::<Vec<_>>()[1..input.len() - 1]
                .iter()
                .collect::<String>();

            let mut index = 0;
            let mut b_count = 0;
            for c in input.chars() {
                index += 1;
                if c == '[' {
                    b_count += 1;
                } else if c == ']' {
                    b_count -= 1;
                }

                if b_count == 0 {
                    break;
                }
            }
            let new_input = input.chars().collect::<Vec<char>>()[index..]
                .iter()
                .collect::<String>();
            let new_index = new_input.find(',').unwrap();
            let (a, b) = input.split_at(index + new_index);
            let b = b.replacen(',', "", 1);
            let left = Num::new(a);
            let right = Num::new(&b);
            Self::Pair {
                left: Box::new(left),
                right: Box::new(right),
            }
        } else {
            let input = input.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let num = input.parse::<i32>().unwrap();
            Self::Val(num)
        }
    }

    fn add(self, other: Num) -> Num {
        Self::Pair {
            left: Box::new(self),
            right: Box::new(other),
        }
    }

    fn get_values(&mut self) -> Vec<&mut i32> {
        let mut values: Vec<&mut i32> = Vec::new();
        match self {
            Num::Val(v) => {
                values.push(v);
            }
            Num::Pair { left, right } => {
                let a = left.get_values();
                let b = right.get_values();
                values.extend(a.into_iter());
                values.extend(b.into_iter());
            }
        }
        values
    }

    fn find_leftmost_explosion(&self, depth: u32, path: String) -> Option<String> {
        if let Num::Pair { left, right } = self {
            if depth == 4 {
                Some(path)
            } else {
                let depth = depth + 1;
                let a = left.find_leftmost_explosion(depth, path.clone() + "l");
                if a.is_some() {
                    return a;
                }
                let b = right.find_leftmost_explosion(depth, path + "r");
                if b.is_some() {
                    return b;
                }
                None
            }
        } else {
            None
        }
    }

    fn explode(&mut self, path: String, left_val: Option<&mut i32>, right_val: Option<&mut i32>) {
        if let Num::Pair { left, right } = self {
            if path.len() != 1 {
                // Not in the pair that contains the exploding pair
                let mut path = path;
                if 'l' == path.remove(0) {
                    let right_vals = right.get_values();
                    let right_val = match right_vals.into_iter().next() {
                        Some(v) => Some(v),
                        None => right_val,
                    };
                    left.explode(path, left_val, right_val);
                } else {
                    let left_vals = left.get_values();
                    let left_val = match left_vals.into_iter().last() {
                        Some(v) => Some(v),
                        None => left_val,
                    };
                    right.explode(path, left_val, right_val);
                }
            } else {
                // In pair containing the exploding pair
                if path.starts_with('l') {
                    // Exploding pair on the left
                    let old = left.get_values();
                    let right = right.get_values();
                    if let Some(right_val) = right.into_iter().next() {
                        *right_val += *old[1]
                    } else if let Some(right_val) = right_val {
                        *right_val += *old[1]
                    }
                    if let Some(left_val) = left_val {
                        *left_val += *old[0];
                    }
                    *left = Box::new(Num::Val(0));
                } else {
                    // Exploding pair on the right
                    let old = right.get_values();
                    let left = left.get_values();
                    if let Some(left_val) = left.into_iter().last() {
                        *left_val += *old[0];
                    } else if let Some(left_val) = left_val {
                        *left_val += *old[0];
                    }
                    if let Some(right_val) = right_val {
                        *right_val += *old[1];
                    }
                    *right = Box::new(Num::Val(0));
                }
            }
        }
    }

    fn find_leftmost_split(&self, path: String) -> Option<String> {
        match self {
            Num::Pair { left, right } => {
                let a = left.find_leftmost_split(path.clone() + "l");
                if a.is_some() {
                    return a;
                }
                let b = right.find_leftmost_split(path + "r");
                if b.is_some() {
                    return b;
                }
                None
            }
            Num::Val(v) => {
                if v >= &10 {
                    Some(path)
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self, path: String) {
        if let Num::Pair { left, right } = self {
            if path.len() != 1 {
                let mut path = path;
                if 'l' == path.remove(0) {
                    left.split(path);
                } else {
                    right.split(path);
                }
            } else {
                // In pair containing splitting value
                if path.starts_with('l') {
                    // value on left
                    if let Num::Val(v) = **left {
                        let v = v as f32;
                        let d = v / 2.0;
                        let a = d.floor();
                        let a = Box::new(Num::Val(a as i32));
                        let b = d.ceil();
                        let b = Box::new(Num::Val(b as i32));
                        *left = Box::new(Num::Pair { left: a, right: b });
                    }
                } else {
                    // value on right
                    if let Num::Val(v) = **right {
                        let v = v as f32;
                        let d = v / 2.0;
                        let a = d.floor();
                        let a = Box::new(Num::Val(a as i32));
                        let b = d.ceil();
                        let b = Box::new(Num::Val(b as i32));
                        *right = Box::new(Num::Pair { left: a, right: b });
                    }
                }
            }
        }
    }

    fn reduce(&mut self) {
        loop {
            // println!("{}", self);
            if let Some(path) = self.find_leftmost_explosion(0, String::new()) {
                self.explode(path, None, None);
            } else if let Some(path) = self.find_leftmost_split(String::new()) {
                // println!("SPLIT :D");
                self.split(path);
            } else {
                break;
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Num::Val(v) => *v,
            Num::Pair { left, right } => {
                let left = left.magnitude();
                let right = right.magnitude();
                (3 * left) + (2 * right)
            }
        }
    }
}
