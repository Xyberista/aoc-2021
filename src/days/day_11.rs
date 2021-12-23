use std::fmt::Display;

use super::super::utils::*;

pub fn day_11() {
    let input = get_input(11);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

type BoardType = Vec<Vec<Option<i32>>>;

fn part_one(input: &str) -> i32 {
    let mut board = Board::new(input);
    let flashes = board.step(100);
    println!("{}", board);
    flashes
}

fn part_two(input: &str) -> i32 {
    let mut board = Board::new(input);
    let mut s = 0;
    while !board.synced() {
        board.step_one();
        s += 1;
    }
    s
}

struct Board {
    board: BoardType,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for c in row {
                let c = match c {
                    Some(v) => *v,
                    None => 0,
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
fn any_flash(board: &BoardType) -> bool {
    for row in board {
        for col in row {
            match col {
                Some(v) => {
                    if *v > 9 {
                        return true;
                    }
                }
                None => continue,
            }
        }
    }
    false
}

impl Board {
    fn new(input: &str) -> Self {
        let mut board = Vec::new();
        for line in input.lines() {
            board.push(
                line.chars()
                    .map(|c| Some(c.to_digit(10).unwrap() as i32))
                    .collect(),
            )
        }
        Self { board }
    }

    fn synced(&self) -> bool {
        for row in &self.board {
            for c in row {
                match c {
                    Some(v) => {
                        if *v != 0 {
                            return false;
                        }
                    }
                    None => continue,
                }
            }
        }
        true
    }

    fn step(&mut self, steps: i32) -> i32 {
        let mut flashes = 0;
        for _ in 0..steps {
            flashes += self.step_one();
        }
        flashes
    }

    fn step_one(&mut self) -> i32 {
        let mut flashes = 0;
        let mut b: BoardType = Vec::new();
        // Step 1 - Increase all by one
        for row in self.board.clone() {
            b.push(row.into_iter().map(|c| Some(c.unwrap() + 1)).collect())
        }
        let mx = self.board[0].len();
        let my = self.board.len();

        let delta = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        while any_flash(&b) {
            for y in 0..my {
                for x in 0..mx {
                    let o = match b[y][x] {
                        Some(v) => v,
                        None => continue,
                    };
                    if o > 9 {
                        b[y][x] = None;
                        flashes += 1;
                        for (dy, dx) in delta {
                            let ny = y as i32 + dy;
                            let nx = x as i32 + dx;
                            if nx >= 0 && nx < mx as i32 && ny >= 0 && ny < my as i32 {
                                let o = b[ny as usize][nx as usize];
                                if let Some(v) = o {
                                    b[ny as usize][nx as usize] = Some(v + 1);
                                }
                            }
                        }
                    }
                }
            }
        }
        // set boards

        for (y, row) in b.into_iter().enumerate() {
            for (x, c) in row.into_iter().enumerate() {
                let v = match c {
                    Some(v) => v,
                    None => 0,
                };
                self.board[y][x] = Some(v);
            }
        }

        flashes
    }
}
