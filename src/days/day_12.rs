use std::collections::{HashMap, HashSet};

use super::super::utils::*;

pub fn day_12() {
    let input = get_input(12);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

type Map<'a> = HashMap<&'a str, Vec<&'a str>>;

fn part_one(input: &str) -> i32 {
    let mut map: Map = HashMap::new();
    for i in input.lines() {
        let (start, end) = i.split_once('-').unwrap();
        map.entry(start).or_insert_with(Vec::new).push(end);
        map.entry(end).or_insert_with(Vec::new).push(start);
    }
    let paths = traverse("start", map, vec!["start"]);
    paths.len() as i32
}

fn is_small(input: &str) -> bool {
    input.chars().all(|c| c.is_lowercase())
}

fn traverse<'a>(current: &str, map: Map<'a>, visited: Vec<&'a str>) -> HashSet<Vec<&'a str>> {
    let mut paths = HashSet::new();
    // println!("{:?}", visited);
    if map.contains_key(&current) {
        for new in map[current].clone() {
            if new.chars().all(|c| c.is_lowercase()) && visited.contains(&new) {
                continue;
            }
            let mut visited = visited.clone();
            visited.push(new);
            if new == "end" {
                paths.insert(visited);
            } else {
                let new_paths = traverse(new, map.clone(), visited.clone());
                paths.extend(new_paths);
            }
        }
    } else if visited.len() >= 2 {
        let index = visited.len() - 2;
        let new = visited[index];
        if !(is_small(new) && visited.contains(&new)) {
            let new_paths = traverse(new, map.clone(), visited.clone());
            paths.extend(new_paths);
        }
    }
    paths
}

fn traverse_2<'a>(
    current: &str,
    map: Map<'a>,
    visited: Vec<&'a str>,
    twice: &'a str,
) -> HashSet<Vec<&'a str>> {
    let mut paths = HashSet::new();
    // println!("{:?}", visited);
    if map.contains_key(&current) {
        for new in map[current].clone() {
            if is_small(new) {
                if new == twice {
                    if visited.iter().filter(|c| **c == twice).count() >= 2 {
                        continue;
                    }
                } else if visited.contains(&new) {
                    continue;
                }
            }
            let mut visited = visited.clone();
            visited.push(new);
            if new == "end" {
                paths.insert(visited);
            } else {
                let new_paths = traverse_2(new, map.clone(), visited.clone(), twice);
                paths.extend(new_paths);
            }
        }
    } else if visited.len() >= 2 {
        let index = visited.len() - 2;
        let new = visited[index];
        if is_small(new) {
            if new == twice && visited.iter().filter(|c| **c == twice).count() >= 2 {
                return paths;
            }
            if visited.contains(&new) {
                return paths;
            }
        }

        let new_paths = traverse_2(new, map.clone(), visited.clone(), twice);
        paths.extend(new_paths);
    }
    paths
}

fn part_two(input: &str) -> i32 {
    let mut map: Map = HashMap::new();
    let mut lower = Vec::new();
    for i in input.lines() {
        let (start, end) = i.split_once('-').unwrap();
        if is_small(start) {
            lower.push(start);
        }
        if is_small(end) {
            lower.push(end);
        }
        map.entry(start).or_insert_with(Vec::new).push(end);
        map.entry(end).or_insert_with(Vec::new).push(start);
    }
    let mut paths = HashSet::new();
    for i in lower {
        if i == "start" || i == "end" {
            continue;
        }
        let p = traverse_2("start", map.clone(), vec!["start"], i);
        paths.extend(p);
    }
    paths.len() as i32
}
