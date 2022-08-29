use super::super::utils::*;

use std::collections::{BinaryHeap, HashMap};

/// (Row from top, Column From left)
type Position = (usize, usize);
type Risk = usize;
type Board = HashMap<Position, Risk>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path {
    total_risk: usize,
    position: Position,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .total_risk
            .cmp(&self.total_risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day_15() {
    let input = get_input(15);
    let board: Board = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row, col), c.to_digit(10).unwrap() as usize))
        })
        .collect::<Board>();
    // println!("{:?}", board);

    println!("{}", part_1(board.clone()));
}

fn part_1(board: Board) -> usize {
    shortest_path(board, (99, 99), 100).unwrap()
}

fn shortest_path(board: Board, goal: Position, max: usize) -> Option<usize> {
    let mut dist: HashMap<Position, Risk> = HashMap::new();
    let mut queue: BinaryHeap<Path> = BinaryHeap::new();

    // start
    dist.insert((0, 0), board[&(0, 0)]);
    queue.push(Path {
        total_risk: 0,
        position: (0, 0),
    });

    while let Some(Path {
        total_risk,
        position,
    }) = queue.pop()
    {
        if position == goal {
            return Some(total_risk);
        }

        if total_risk > dist[&position] {
            continue;
        }

        for new_position in get_new_positions(position, max) {
            let next = Path {
                total_risk: total_risk + board[&new_position],
                position: new_position,
            };

            if next.total_risk < *dist.entry(new_position).or_insert(usize::MAX) {
                queue.push(next);
                dist.insert(new_position, next.total_risk);
            }
        }
    }

    None
}

fn get_new_positions((row, col): Position, max: usize) -> Vec<Position> {
    let mut positions = Vec::new();
    if row > 0 {
        positions.push((row - 1, col));
    }
    if row < max - 1 {
        positions.push((row + 1, col));
    }
    if col > 0 {
        positions.push((row, col - 1));
    }
    if col < max - 1 {
        positions.push((row, col + 1));
    }
    positions
}
