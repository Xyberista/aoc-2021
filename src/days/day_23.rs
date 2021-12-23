use super::super::utils::*;

pub fn day_23() {
    let input = get_input(23);
    println!("Part 1: {}", part_one(&input));
    // println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(23);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_two() {
        let input = get_input(23);
        // assert_eq!(part_two(&input), 0);
    }
}