use super::super::utils;

type Report = Vec<usize>;

pub fn get_report() -> Report {
    utils::get_input(1)
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn day_1(output: utils::Output) {
    let report = get_report();
    run(report, output);
}

fn run(report: Report, output: utils::Output) -> Option<(usize, usize)> {
    let one = part_1(&report);
    let two = part_2(&report);
    match output {
        utils::Output::Return => Some((one, two)),
        utils::Output::Print => {
            println!("Part 1: {one}");
            println!("Part 2: {two}");
            None
        }
    }
}

pub fn part_1(report: &Report) -> usize {
    report.windows(2).filter(|&win| win[1] > win[0]).count()
}

pub fn part_2(report: &Report) -> usize {
    report.windows(4).filter(|&win| win[3] > win[0]).count()
}
