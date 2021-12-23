use std::collections::{HashSet, BTreeMap};

use super::super::utils::*;

type Point = (usize, usize);
#[derive(Clone, Debug)]
enum Fold {
    Y(usize),
    X(usize),
}

pub fn day_13() {
    let input = get_input(13);
    let mut map: HashSet<Point> = HashSet::new();
    let (points, rest) = input.split_once("\n\n").unwrap();
    let mut mx = 0;
    let mut my = 0;
    for p in points.lines() {
        let (x, y) = p.split_once(',').unwrap();
        let x = x.parse().unwrap();
        mx = mx.max(x);
        let y = y.parse().unwrap();
        my = my.max(y);
        map.insert((x, y));
    }
    let mut folds = Vec::new();
    for f in rest.lines() {
        if let Some((_, y)) = f.split_once("y=") {
            folds.push(Fold::Y(y.parse().unwrap()));
        } else if let Some((_, x)) = f.split_once("x=") {
            folds.push(Fold::X(x.parse().unwrap()));
        }
    }

    let sizes = get_paper_after_fold(&folds, mx, my);
    let one = get_paper_after_fold(&[folds[0].clone()], mx+1, my+1);
    // println!("{} {}", paper.len(), paper[0].len());

    println!("Part 1: {}", part_one(&map, &folds[0], one));
    println!("Part 1: {}", part_two(&map, &folds, sizes));
}

fn get_paper_after_fold(folds: &[Fold], mx: usize, my: usize) -> (usize, usize) {
    let mut sy = my;
    let mut sx = mx;
    for f in folds {
        match f {
            Fold::Y(y) => {
                sy -= y - 1;
            }
            Fold::X(x) => {
                sx -= x - 1;
            }
        }
    }
    (sx, sy)
}

fn part_one(paper: &HashSet<Point>, fold: &Fold, (sx, sy): (usize, usize)) -> usize {
    let mut new_paper: HashSet<Point> = HashSet::new();
    match *fold {
        Fold::Y(fy) => {
            for (x, y) in paper.clone() {
                if y > fy {
                    let diff = y - fy;
                    let ny = fy - diff;
                    new_paper.insert((x, ny));
                } else {
                    new_paper.insert((x, y));
                }
            }
        }
        Fold::X(fx) => {
            for (x, y) in paper.clone() {
                if x > fx {
                    let diff = x - fx;
                    let nx = fx - diff;
                    new_paper.insert((nx, y));
                } else {
                    new_paper.insert((x, y));
                }
            }
        }
    }

    let mut dots = 0;
    for (x, y) in new_paper {
        if x < sx+1 && y < sy+1 {
            dots += 1;
        }
    }
    dots
}

fn part_two(paper: &HashSet<Point>, folds: &[Fold], (sx, sy): (usize, usize)) -> usize {
    let mut paper = paper.clone();
    for fold in folds {
        let mut new_paper = HashSet::new();
        match fold {
            Fold::Y(fy) => {
                for (x, y) in paper {
                    if y > *fy {
                        let diff = y - fy;
                        let ny = fy - diff;
                        new_paper.insert((x, ny));
                    } else {
                        new_paper.insert((x, y));
                    }
                }
            }
            Fold::X(fx) => {
                for (x, y) in paper {
                    if x > *fx {
                        let diff = x - fx;
                        let nx = fx - diff;
                        new_paper.insert((nx, y));
                    } else {
                        new_paper.insert((x, y));
                    }
                }
            }
        }
        paper = new_paper;
    }

    let mut p: BTreeMap<usize, BTreeMap<usize, char>> = BTreeMap::new();
    for (x, y) in paper {
        p.entry(y).or_default().insert(x, '#');
    }
    let mut y = 0;
    let mut x = 0;
    println!("{}", sy);
    while y < sy {
        while x < sx {
            if let Some(a) = p.get(&y) {
                if let Some(b) = a.get(&x) {
                    print!("{}", b);
                } else {
                    print!(".")
                }
            } else {
                print!(".")
            }
            x += 1;
        }
        println!("");
        y += 1;
        x = 0;
    }

    let dots = 0;
    dots
}
