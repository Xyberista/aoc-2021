use std::collections::{BTreeMap, HashSet};

use super::super::utils::*;

/// (X, Y)
type Point = (usize, usize);
type Map = BTreeMap<Point, usize>;

pub fn day_15(output: Output) {
    let input = get_input(15);
    let mut map: Map = BTreeMap::new();
    let mut h = 0;
    let mut v = 0;
    for (y, line) in input.lines().enumerate() {
        h = line.len();
        v += 1;
        for (x, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap() as usize;
            map.insert((x, y), n);
        }
    }
    
    run(map, (h, v), output).unwrap();
}

fn run(map: Map, size: (usize, usize), output: Output) -> Option<(usize, usize)> {
    let one = part_one(&map, size);
    let two = part_two(map, size);
    match output {
        Output::Return => Some((one, two)),
        Output::Print => {
            println!("Part 1: {one}");
            println!("Part 2: {two}");
            None
        }
    }
}

fn traverse(map: &Map, size: (usize, usize)) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut distances: BTreeMap<Point, usize> = BTreeMap::new();
    let mut path: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
    path.insert((0, 0), vec![]);

    for y in 0..size.1 {
        for x in 0..size.0 {
            distances.insert((x, y), usize::MAX);
        }
    }
    distances.insert((0, 0), 0);
    unvisited.insert((0, 0));

    loop {
        if !unvisited.is_empty() {
            let mut min = usize::MAX;
            let mut node: Point = (0, 0);
            for n in &unvisited {
                let d = distances[n];
                if d < min {
                    node = *n;
                    min = d;
                }
            }
            unvisited.remove(&node);
            let (x, y) = node;
            let d = distances[&(x, y)];

            if !visited.contains(&node) {
                visited.insert(node);
                let deltas: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
                // update adjacent
                for (dx, dy) in deltas {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < size.0 as i32 && ny >= 0 && ny < size.1 as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        let new_node = (nx, ny);
                        if !visited.contains(&new_node) {
                            unvisited.insert((nx, ny));
                        }
                        let new_d = d + map[&(nx, ny)];
                        if new_d < distances[&(nx, ny)] {
                            distances.insert((nx, ny), new_d);

                            let mut p = path[&(x, y)].clone();
                            p.push((nx, ny));
                            path.insert((nx, ny), p);
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    // println!("{:?}", path[&(size.0 - 1, size.1 - 1)]);
    distances[&(size.0 - 1, size.1 - 1)]
}

fn part_one(map: &Map, size: (usize, usize)) -> usize {
    traverse(map, size)
}

fn part_two(map: Map, size: (usize, usize)) -> usize {
    let mut new_map: BTreeMap<Point, usize> = BTreeMap::new();
    // tile board
    for dy in 0..5 {
        // tile a row
        for dx in 0..5 {
            for ((x, y), v) in &map {
                let nx = dx * size.0 + *x;
                let ny = dy * size.1 + *y;
                let mut nv = (*v) + dx as usize + dy as usize;
                while nv > 9 {
                    nv -= 9;
                }

                new_map.insert((nx, ny), nv);
            }
        }
    }
    // println!("{:?}", new_map);
    // for y in 0..size.0 * 5 {
    //     for x in 0..size.1 * 5 {
    //         print!("{}", new_map[&(x, y)]);
    //     }
    //     println!();
    // }
    traverse(&new_map, (size.0 * 5, size.1 * 5))
}
