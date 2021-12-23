use std::collections::HashMap;

use super::super::utils::*;

pub fn day_14() {
    let input = get_input(14);
    let (template, rest) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<&str, char> = HashMap::new();
    for r in rest.lines() {
        let (pair, element) = r.split_once(" -> ").unwrap();
        rules.insert(pair, element.chars().next().unwrap());
    }
    println!("Part 1: {}", part_one(template, &rules));
    println!("Part 2: {}", part_two(template, &rules));
}

fn step_one(template: String, rules: &HashMap<&str, char>) -> String {
    let mut c = template.chars().collect::<Vec<_>>();
    let mut changes: Vec<(char, usize)> = Vec::new();
    let mut start = 0;
    let mut end = 1;
    let max = template.len();
    while end < max {
        let pair = c[start..=end].into_iter().collect::<String>();
        if rules.contains_key(pair.as_str()) {
            changes.push((rules[pair.as_str()], end));
        }
        start += 1;
        end += 1;
    }
    while changes.len() != 0 {
        let (new, index) = changes.remove(0);
        c.insert(index, new);
        changes = changes.into_iter().map(|(i,v)| (i, v+1)).collect();
    }
    c.into_iter().collect()
}

fn part_one(template: &str, rules: &HashMap<&str, char>) -> i32 {
    let mut template = template.to_owned();
    for _ in 0..10 {
        template = step_one(template, rules);
    }
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_default() += 1;
    }
    let counts = counts.values().cloned();
    let max = counts.clone().max().unwrap();
    let min = counts.min().unwrap();
    max - min
}

fn part_two(template: &str, rules: &HashMap<&str, char>) -> i64 {
    let c = template.chars().collect::<Vec<_>>();
    // comb, count
    let mut comb: HashMap<String, i64> = HashMap::new();
    for p in c[..].windows(2) {
        let s = p.into_iter().collect::<String>();
        *comb.entry(s).or_default() += 1;
    }
    for _ in 0..40 {
        let mut new: HashMap<String, i64> = HashMap::new();
        for (k, v) in comb {
            if rules.contains_key(k.as_str()) {
                let n = rules[k.as_str()];
                let mut a = k.clone();
                a.pop();
                a.push(n);
                *new.entry(a).or_default() += v;

                let mut b = k.clone();
                b.remove(0);
                b.insert(0, n);
                *new.entry(b).or_default() += v;
            }
        }
        comb = new;
    }

    // find occurrances
    let mut count: HashMap<char, i64> = HashMap::new();
    for (s, n) in comb {
        let mut s = s;
        let b = s.pop().unwrap();
        let a = s.pop().unwrap();
        *count.entry(b).or_default() += n;
        *count.entry(a).or_default() += n;
    }
    let v = count.values();
    let max = (*v.clone().max().unwrap() as f64 / 2.0).ceil();
    let min = (*v.min().unwrap() as f64 / 2.0).ceil();
    (max - min) as i64
}
