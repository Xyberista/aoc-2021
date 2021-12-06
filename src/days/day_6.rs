use super::super::utils::*;

pub fn day_6() {
    let input = get_input(6);
    let ages: [i64; 9] = parse(&input);
    println!("Part 1: {}", calc(80, ages));
    println!("Part 2: {}", calc(256, ages));
}

fn parse(input: &str) -> [i64; 9] {
    let mut ages = [0; 9];
    for a in input.trim().split(',') {
        let a = a.parse::<usize>().unwrap();
        ages[a] += 1;
    }
    ages
}
fn calc(days: i32, ages: [i64; 9]) -> i64 {
    let mut ages = ages;
    for _ in 0..days {
        ages.rotate_left(1);
        ages[6] += ages[8];
    }
    ages.iter().sum()
}