use std::collections::HashMap;

use super::super::utils::*;

pub fn day_4() {
    let input = get_input(4);
    // println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

type Board = Vec<Vec<i32>>;

fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = lines[0].split(',').map(|s| s.parse::<i32>().unwrap());
    let mut boards: Vec<Board> = Vec::new();
    for board in lines[2..].split(|c| c.is_empty()) {
        let mut b: Board = Vec::new();
        for &line in board {
            let line = line.replace("  ", " ");
            let line = line.trim();
            b.push(line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect());
        }
        boards.push(b);
    }
    // println!("{:?}", boards);
    let mut win = Vec::new();
    let mut f: i32 = -1;
    for n in numbers {
        let mut n_boards = Vec::new();
        for board in boards {
            let n_b = change(board, n);
            if winning_board(&n_b) {
                win = n_b;
                f = n;
                break;
            }
            n_boards.push(n_b);
        }
        if !win.is_empty() {
            break;
        }
        boards = n_boards;
    }
    let s = get_sum(&win);
    println!("{}", s * f);
    0
}

fn part_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = lines[0].split(',').map(|s| s.parse::<i32>().unwrap());
    let mut boards: Vec<Board> = Vec::new();
    for board in lines[2..].split(|c| c.is_empty()) {
        let mut b: Board = Vec::new();
        for &line in board {
            let line = line.replace("  ", " ");
            let line = line.trim();
            b.push(line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect());
        }
        boards.push(b);
    }
    // println!("{:?}", boards);
    let mut last = Vec::new();
    let mut f: i32 = -1;
    for n in numbers {
        let mut n_boards = Vec::new();
        for board in boards.clone() {
            let n_b = change(board, n);
            if winning_board(&n_b) {
                continue;
            } else {
                n_boards.push(n_b);
            }
        }
        if n_boards.is_empty() {
            let l = boards[0].clone();
            last = change(l, n);
            f = n;
        }
        if !last.is_empty() {
            break;
        }
        boards = n_boards;
    }
    let s = get_sum(&last);
    println!("{}", s);
    println!("{}", s * f);
    0
}
fn winning_board(board: &Board) -> bool {
    let mut cols: HashMap<usize, i32> = HashMap::new();
    for row in board {
        if row.iter().all(|&i| i == -1) {
            return true;
        }
        for (x, c) in row.iter().enumerate() {
            if *c == -1 {
                let entry = cols.entry(x).or_insert(0);
                *entry += 1;
            }
        }
    }
    cols.values().any(|v| *v == 5)
}

fn get_sum(board: &Board) -> i32 {
    let mut s = 0;
    for r in board {
        for &i in r {
            if i != -1 {
                s += i;
            }
        }
    }
    s
}

fn change(board: Board, number: i32) -> Board {
    let mut board = board;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == number {
                board[y][x] = -1;
            }
        }
    }
    board
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(4);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_two() {
        // let input = get_input(4);
        // assert_eq!(part_two(&input), 0);
    }
}
