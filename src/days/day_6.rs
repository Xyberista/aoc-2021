use std::collections::{HashMap, BTreeMap};

use super::super::utils::*;

pub fn day_6() {
    let input = get_input(6);
    // println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}",  two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut ages: Vec<i32> = input.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut day = 0;
    while day < 80 {
        let mut n_ages: Vec<i32> = Vec::new();
        for a in ages {
            if a == 0 {
                n_ages.push(6);
                n_ages.push(8);
            } else {
                n_ages.push(a - 1);
            }
        }
        ages = n_ages;
        day += 1;
    }
    println!("{}", ages.len());
    0
}

fn two(input: &str) -> i32 {
    let mut ages: BTreeMap<i32, i64> = BTreeMap::new();
    for a in input.split(',') {
        let a = a.parse::<i32>().unwrap();
        let e = ages.entry(a).or_default();
        *e += 1;
    }
    for _ in 0..256 {
        let mut n_ages: BTreeMap<i32, i64> = BTreeMap::new();
        for (k,v) in ages {
            if k == 0 {
                n_ages.insert(8, v);
                let e = n_ages.entry(6).or_default();
                *e += v;
            } else {
                let e = n_ages.entry(k-1).or_default();
                *e += v;
            }
        }
        ages = n_ages;
    }
    println!("{:?}", ages.values().sum::<i64>());
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(6);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_two() {
        let input = get_input(6);
        // assert_eq!(part_two(&input), 0);
    }
}