use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();
    file.read_to_string(&mut buf).unwrap();
    let input = buf.lines().map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut min = 0;
    let mut max = 2;
    let mut p = 0;
    let mut t = 0;
    while max < input.len() {
        let c = input[min..=max].iter().sum();
        if p != 0 {
            if c > p {
                t += 1;
            }
        }
        min += 1;
        max += 1;
        p = c;
    }
    println!("{}", t);
}

fn one() {
    let mut buf = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();
    file.read_to_string(&mut buf).unwrap();
    let mut input = buf.lines();
    let mut t = 0;
    let mut p = -1;
    while let Some(current) = input.next() {
        let c = current.parse::<i32>().unwrap();
        if c > p {
            if p != -1 {
                t += 1;
            }
        }
        p = c;
    }
    println!("{}", t)
}
