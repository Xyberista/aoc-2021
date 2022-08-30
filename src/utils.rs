use std::io::prelude::*;

pub fn get_input(day: i32) -> String {
    let mut buf = String::new();
    let mut file =
        std::fs::File::open("./input/day_".to_string() + &day.to_string() + ".txt").unwrap();
    file.read_to_string(&mut buf).unwrap();
    buf
}

pub enum Output {
    Return,
    Print,
}