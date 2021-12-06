struct Position {
    x: i32,
    y: i32,
}
fn day_2() {
    let mut buf = String::new();
    let mut file = std::fs::File::open("./input/day_2.txt").unwrap();
    file.read_to_string(&mut buf);
    two(&buf);
}

fn one(input: &str) {
    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        let mut s = line.split(" ");
        let a = s.next().unwrap();
        let b = s.next().unwrap().parse::<i32>().unwrap();
        match a {
            "forward" => x += b,
            "down" => y += b,
            "up" => y -= b,
            _ => unimplemented!()
        }
    }
    println!("{}", x * y);
}
fn two(input: &str) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut s = line.split(" ");
        let a = s.next().unwrap();
        let b = s.next().unwrap().parse::<i32>().unwrap();
        match a {
            "down" => aim += b,
            "up" => aim -= b,
            "forward" => {
                x += b;
                y += aim * b;
            }
            _ => unimplemented!()
        }
    }
    println!("{}", x * y);
}