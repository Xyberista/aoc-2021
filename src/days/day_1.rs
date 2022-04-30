use super::super::utils::*;

pub fn day_1() {
    let input = get_input(1);
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let lines: Vec<i32> = input.lines().map(|c| c.parse::<i32>().unwrap()).collect();
    let mut t = 0;
    for w in lines.windows(2) {
        if w[0] < w[1] {
            t += 1;
        }
    }
    t
}
fn part_two(input: &str) -> i32 {
    let lines: Vec<i32> = input.lines().map(|c| c.parse::<i32>().unwrap()).collect();
    let mut t = 0;
    let mut windows = lines.windows(3);
    let mut p = windows.next().unwrap().iter().sum::<i32>();
    for w in windows {
        let s: i32 = w.iter().sum();
        if s > p {
            t += 1;
        }
        p = s;
    }
    t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(1);
        assert_eq!(part_one(&input), 1228);
    }

    #[test]
    fn test_two() {
        let input = get_input(1);
        // change one -> two
        assert_eq!(part_two(&input), 1257);
    }
}
