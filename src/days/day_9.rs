use std::collections::HashSet;

use super::super::utils::*;

pub fn day_9() {
    let input = get_input(9);
    let mut map = Vec::new();
    for line in input.lines() {
        let v = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(v);
    }
    println!("Part 1: {}", part_one(&map));
    println!("Part 2: {}", part_two(&map));
}

fn part_one(input: &[Vec<u32>]) -> u32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let m_x = input[0].len();
    let m_y = input.len();
    let mut risk = 0;
    while x < m_x && y < m_y {
        let current = input[y][x];
        let mut adj = Vec::new();
        if x > 0 {
            let a = input[y][x - 1];
            adj.push(a);
        }
        if x < m_x - 1 {
            let a = input[y][x + 1];
            adj.push(a);
        }
        if y > 0 {
            let a = input[y - 1][x];
            adj.push(a);
        }
        if y < m_y - 1 {
            let a = input[y + 1][x];
            adj.push(a);
        }
        if adj.iter().all(|c| *c > current) {
            risk += current + 1;
        }

        if x == m_x - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    risk
}

/// Returns Vec<x, y>
fn find_low(map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let m_x = map[0].len();
    let m_y = map.len();
    while x < m_x && y < m_y {
        let current = map[y][x];
        let mut adj = Vec::new();
        if x > 0 {
            let a = map[y][x - 1];
            adj.push(a);
        }
        if x < m_x - 1 {
            let a = map[y][x + 1];
            adj.push(a);
        }
        if y > 0 {
            let a = map[y - 1][x];
            adj.push(a);
        }
        if y < m_y - 1 {
            let a = map[y + 1][x];
            adj.push(a);
        }
        if adj.iter().all(|c| *c > current) {
            points.push((x, y))
        }

        if x == m_x - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    points
}

fn part_two(map: &[Vec<u32>]) -> usize {
    let low = find_low(map);
    // println!("LOWS: {:?}", low);
    let mut sizes = Vec::new();
    for (x, y) in low {
        let h = traverse(map, HashSet::new(), map[y][x], x, y);
        sizes.push(h.len());
    }
    sizes.sort();
    sizes.into_iter().rev().take(3).product()
}

fn find_adj(map: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let m_x = map[0].len();
    let m_y = map.len();
    let mut adj = Vec::new();
    if x > 0 {
        adj.push((x - 1, y));
    }
    if x < m_x - 1 {
        adj.push((x + 1, y));
    }
    if y > 0 {
        adj.push((x, y - 1));
    }
    if y < m_y - 1 {
        adj.push((x, y + 1));
    }
    adj
}

fn traverse(
    map: &[Vec<u32>],
    visited: HashSet<(usize, usize)>,
    prev: u32,
    x: usize,
    y: usize,
) -> HashSet<(usize, usize)> {
    let mut visited = visited;
    let current = map[y][x];
    if current < prev || current == 9 {
        return visited;
    }
    let adj = find_adj(map, x, y);
    // println!("ADJ: {:?}", adj);
    visited.insert((x, y));
    for (nx, ny) in adj {
        // println!("{} {}", nx, ny);
        if !visited.contains(&(nx, ny)) {
            let new = traverse(map, visited.clone(), current, nx, ny);
            visited.extend(new.into_iter());
        }
    }
    visited
}
