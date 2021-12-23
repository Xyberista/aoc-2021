use super::super::utils::*;

pub fn day_2() {
    let input = get_input(2);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut h = 0;
    let mut d = 0;
    for line in input.lines() {
        let mut s = line.split(" ");
        let a = s.next().unwrap();
        let b = s.next().unwrap().parse::<i32>().unwrap();
        match a {
            "forward" => h += b,
            "down" => d += b,
            "up" => d -= b,
            _ => panic!(),
        }
    }
    h * d
}

fn part_two(input: &str) -> i32 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut s = line.split(" ");
        let a = s.next().unwrap();
        let b = s.next().unwrap().parse::<i32>().unwrap();
        match a {
            "forward" => {
                h += b;
                d += aim * b;
            },
            "down" => aim += b,
            "up" => aim -= b,
            _ => panic!(),
        }
    }
    h * d
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(1);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_two() {
        // let input = get_input(1);
        // assert_eq!(part_two(&input), 0);
    }
}