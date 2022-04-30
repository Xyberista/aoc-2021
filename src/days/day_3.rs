use super::super::utils::*;

pub fn day_3() {
    let input = get_input(3);
    // println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut gamma = String::new();
    let mut epsilon = String::new();
    let mut ones = [0; 12];
    let mut zeros = [0; 12];
    for line in input.lines() {
        for (x, c) in line.chars().enumerate() {
            if c == '1' {
                ones[x] += 1;
            } else {
                zeros[x] += 1;
            }
        }
    }
    let ones = ones.to_vec();
    println!("{:?}", ones);
    for (a, b) in ones.into_iter().zip(zeros) {
        if a > b {
            gamma += "1";
            epsilon += "0"
        } else {
            gamma += "0";
            epsilon += "1"
        }
    }
    let g = u32::from_str_radix(&gamma, 2).unwrap();
    let e = u32::from_str_radix(&epsilon, 2).unwrap();
    g * e
}
fn part_two(input: &str) -> u32 {
    let mut ogr: Vec<&str> = input.lines().collect();
    let mut position = 0;
    loop {
        let most = get_most(ogr.clone());
        let f = filter_once(most.clone(), ogr, position);
        ogr = f;
        if ogr.len() <= 1 {
            break;
        }
        position += 1;
    }
    println!("{:?}", &ogr);

    let mut csr: Vec<&str> = input.lines().collect();
    let mut position = 0;
    loop {
        let least = get_least(csr.clone());
        let f = filter_once(least.clone(), csr, position);
        csr = f;
        if csr.len() <= 1 {
            break;
        }
        position += 1;
    }
    let ogr = u32::from_str_radix(&ogr.concat(), 2).unwrap();
    let csr = u32::from_str_radix(&csr.concat(), 2).unwrap();
    println!("{:?}", ogr);
    println!("{:?}", csr);
    ogr * csr
}

fn get_most<'a>(possible: Vec<&'a str>) -> Vec<i32> {
    let mut ones = [0; 12];
    let mut zeros = [0; 12];
    for line in possible {
        for (x, c) in line.chars().enumerate() {
            if c == '1' {
                ones[x] += 1;
            } else {
                zeros[x] += 1;
            }
        }
    }
    let most: Vec<i32> = ones
        .iter()
        .zip(zeros)
        .map(|(&a, b)| {
            if a > b {
                1
            } else if b > a {
                0
            } else {
                1
            }
        })
        .collect();
    most
}

fn get_least<'a>(possible: Vec<&'a str>) -> Vec<i32> {
    let mut ones = [0; 12];
    let mut zeros = [0; 12];
    for line in possible {
        for (x, c) in line.chars().enumerate() {
            if c == '1' {
                ones[x] += 1;
            } else {
                zeros[x] += 1;
            }
        }
    }
    let most: Vec<i32> = ones
        .iter()
        .zip(zeros)
        .map(|(&a, b)| {
            if a > b {
                0
            } else if b > a {
                1
            } else {
                0
            }
        })
        .collect();
    most
}

fn filter_once<'a>(most: Vec<i32>, possible: Vec<&'a str>, position: usize) -> Vec<&'a str> {
    let mut pos = Vec::new();
    for p in possible {
        let chars = p.chars().collect::<Vec<_>>();
        if chars[position].to_string() == most[position].to_string() {
            pos.push(p);
        }
    }
    pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = get_input(3);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_two() {
        // let input = get_input(3);
        // assert_eq!(part_two(&input), 0);
    }
}
