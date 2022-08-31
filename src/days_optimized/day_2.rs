use crate::utils::Output;

use super::super::utils;

type Ops = Vec<Op>;

pub enum Op {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let mut p = s.split(' ');
        match p.next().unwrap() {
            "forward" => Op::Forward(p.next().unwrap().parse().unwrap()),
            "down" => Op::Down(p.next().unwrap().parse().unwrap()),
            "up" => Op::Up(p.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub fn get_instructions() -> Ops {
    utils::get_input(2).lines().map(Op::from).collect()
}

pub fn day_2(output: Output) {
    let instructions = get_instructions();
    run(instructions, output);
}

fn run(ops: Ops, output: Output) -> Option<(usize, usize)> {
    let one = part_1(&ops);
    let two = part_2(&ops);
    match output {
        utils::Output::Return => Some((one, two)),
        utils::Output::Print => {
            println!("Part 1: {one}");
            println!("Part 2: {two}");
            None
        }
    }
}

pub fn part_1(ops: &Ops) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for op in ops {
        match op {
            Op::Forward(x) => horizontal += x,
            Op::Down(x) => depth += x,
            Op::Up(x) => depth -= x,
        }
    }
    horizontal * depth
}

pub fn part_2(ops: &Ops) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for op in ops {
        match op {
            Op::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Op::Down(x) => aim += x,
            Op::Up(x) => aim -= x,
        }
    }
    horizontal * depth
}
