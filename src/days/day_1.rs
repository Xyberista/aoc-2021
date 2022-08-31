use super::super::utils::*;

pub fn input() -> Vec<i32> {
    get_input(1)
        .lines()
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

pub fn day_1() {
    let input = input();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

pub fn part_one(lines: &[i32]) -> i32 {
    let mut t = 0;
    for w in lines.windows(2) {
        if w[0] < w[1] {
            t += 1;
        }
    }
    t
}
pub fn part_two(lines: &[i32]) -> i32 {
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
        let input = input();
        assert_eq!(part_one(&input), 1228);
    }

    #[test]
    fn test_two() {
        let input = input();
        // change one -> two
        assert_eq!(part_two(&input), 1257);
    }
}
