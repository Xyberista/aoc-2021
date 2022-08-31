use super::super::utils::*;

type Image = Vec<Vec<char>>;

pub fn day_20() {
    let input = get_input(20);
    let (algo, rest) = input.split_once("\n\n").unwrap();
    let algo = algo.chars().collect::<Vec<char>>();
    let mut image: Image = Vec::new();
    for line in rest.lines() {
        let line = line.trim();
        let row = line.chars().collect::<Vec<_>>();
        image.push(row);
    }
    println!("Part 1: {}", part_one(&algo, image.clone()));
    println!("Part 2: {}", part_two(&algo, image));
}

fn print_image(image: &Image) {
    for row in image {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn expand_image(_algo: &[char], image: Image, steps: usize) -> Image {
    let f = if steps % 2 == 0 { '.' } else { '#' };
    let empty = vec![f; image.len() + 2];
    let mut new: Image = vec![empty.clone()];
    for line in image {
        let mut line = line;
        line.insert(0, f);
        line.push(f);
        new.push(line);
    }
    new.push(empty);
    // println!("{:?}", new);
    new
}

fn step_one(algo: &[char], image: Image, steps: usize) -> Image {
    let image = expand_image(algo, image, steps);
    let mut new: Image = Vec::new();
    // (row, col)
    let square = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let my = image.len();
    let mx = image[0].len();
    for y in 0..my {
        let mut row = Vec::new();
        for x in 0..mx {
            let mut bin = String::new();
            for (dy, dx) in square {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if nx >= 0 && nx < mx as i32 && ny >= 0 && ny < my as i32 {
                    bin.push(image[ny as usize][nx as usize]);
                } else {
                    let c = if steps % 2 == 0 { '.' } else { '#' };
                    bin.push(c);
                }
            }
            let bin = bin
                .chars()
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect::<String>();
            let bin = usize::from_str_radix(&bin, 2).unwrap();
            let new_char = algo[bin];
            row.push(new_char);
        }
        new.push(row);
    }
    new
}

fn part_one(algo: &[char], image: Image) -> i32 {
    let one = step_one(algo, image, 0);
    let two = step_one(algo, one, 1);
    let mut t = 0;
    for r in two {
        for c in r {
            if c == '#' {
                t += 1;
            }
        }
    }
    t
}

fn part_two(algo: &[char], image: Image) -> i32 {
    let mut image = image;
    for i in 0..50 {
        image = step_one(algo, image, i);
    }
    let mut t = 0;
    for r in image {
        for c in r {
            if c == '#' {
                t += 1;
            }
        }
    }
    t
}
