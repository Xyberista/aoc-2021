use std::collections::HashMap;

#[derive(Clone)]
struct Player {
    position: i64,
    score: i64,
}

impl Player {
    fn new(position: i64) -> Self {
        Self { position, score: 0 }
    }

    fn add_turn(&mut self, spaces_moved: i64) {
        let mut pos = self.position + spaces_moved;
        while pos > 10 {
            pos -= 10;
        }
        self.position = pos;
        self.score += pos;
    }
}

pub fn day_21() {
    let players: Vec<Player> = vec![Player::new(4), Player::new(3)];
    println!("Part 1: {}", part_one(&players));
    println!("Part 2: {}", part_two(players));
}

fn die_one_total(start: i64) -> i64 {
    (0..3).map(|n| start + n).sum()
}

fn part_one(input: &[Player]) -> i64 {
    let mut die = 1;
    let mut players = input.to_owned();
    let mut p = 0;
    let mut die_rolls = 0;
    loop {
        if players.iter().any(|v| v.score >= 1000) {
            break;
        }
        let new = die_one_total(die);
        // println!("{}", new);
        die += 3;
        let player = &mut players[p];
        player.add_turn(new);
        // println!("{} moves to {}", p + 1, player.position);
        p = if p == 0 { 1 } else { 0 };
        die_rolls += 3;
    }
    let lost = players[p].clone();
    lost.score * die_rolls
}

/// (p1 wins, p2 wins)
fn run(players: &[Player], turn: usize, freq: &HashMap<i64, i64>) -> (i64, i64) {
    let mut p1 = 0;
    let mut p2 = 0;
    for t in 3..=9 {
        let mut players = players.to_vec();
        players[turn].add_turn(t);
        if players[turn].score >= 21 {
            match turn {
                0 => p1 += freq[&t],
                1 => p2 += freq[&t],
                _ => unreachable!(),
            }
        } else {
            let turn = match turn {
                0 => 1,
                1 => 0,
                _ => unreachable!(),
            };
            let (np1, np2) = run(&players, turn, freq);
            p1 += np1 * freq[&t];
            p2 += np2 * freq[&t];
        }
    }
    (p1, p2)
}

fn part_two(players: Vec<Player>) -> i64 {
    let freq: HashMap<i64, i64> =
        HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]);
    let (p1, p2) = run(&players, 0, &freq);
    p1.max(p2)
}
