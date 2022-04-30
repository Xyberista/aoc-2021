use core::fmt;
use std::collections::{BinaryHeap, HashSet};

/*
#############
#...........#
###D#C#D#B###
  #B#A#A#C#
  #########
*/

// last is first out
type Room = Vec<char>;

#[derive(Debug, Clone, PartialEq, Hash)]
struct M {
    map: Vec<P>,
    energy: usize,
}

impl M {
    fn new(size: usize, a: Room, b: Room, c: Room, d: Room) -> Self {
        let mut map = Vec::new();
        let a = R::new(size, 'A', a);
        let b = R::new(size, 'B', b);
        let c = R::new(size, 'C', c);
        let d = R::new(size, 'D', d);
        let rooms: Vec<Option<R>> = Vec::from([
            None,
            None,
            Some(a),
            None,
            Some(b),
            None,
            Some(c),
            None,
            Some(d),
            None,
            None,
        ]);
        for r in rooms {
            map.push(P::new('.', r));
        }

        Self { map, energy: 0 }
    }

    fn is_done(&self) -> bool {
        for r in &self.map[2].room {
            if r.room.len() != r.size || !r.room.iter().all(|c| c == &r.occ) {
                return false;
            }
        }
        for r in &self.map[4].room {
            if r.room.len() != r.size || !r.room.iter().all(|c| c == &r.occ) {
                return false;
            }
        }
        for r in &self.map[6].room {
            if r.room.len() != r.size || !r.room.iter().all(|c| c == &r.occ) {
                return false;
            }
        }
        for r in &self.map[8].room {
            if r.room.len() != r.size || !r.room.iter().all(|c| c == &r.occ) {
                return false;
            }
        }
        true
    }

    fn valid_moves(&self) -> Vec<Self> {
        // println!("{}", self);
        let mut moves: Vec<Self> = Vec::new();
        'hall: for (pos, p) in self.map.iter().enumerate() {
            if let Some(old_r) = &p.room {
                // room contained here
                if old_r.room.is_empty() || old_r.room.iter().all(|c| c == &old_r.occ) {
                    continue;
                } else {
                    let mut r = old_r.clone();
                    let l = r.room.pop().unwrap();
                    let mut steps = 0;
                    // steps to immediately outside the room
                    let steps_from_room = r.size - r.room.len();
                    steps += steps_from_room;
                    let mut new_pos = pos.clone();
                    // check left of room
                    while new_pos > 0 {
                        new_pos -= 1;
                        steps += 1;
                        let mut new_p = self.map[new_pos].clone();
                        if new_p.occ != '.' {
                            break;
                        } else {
                            if new_p.room.is_some() {
                                continue;
                            } else {
                                new_p.occ = l;
                                let mut new = self.clone();
                                new.map[new_pos] = new_p;
                                let mut old_p = p.clone();
                                old_p.room = Some(r.clone());
                                new.map[pos] = old_p;
                                let multiplier = match l {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!(),
                                };
                                new.energy += steps * multiplier;
                                moves.push(new)
                            }
                        }
                    }
                    new_pos = pos.clone();
                    steps = steps_from_room;
                    // check right of room
                    while new_pos < 10 {
                        new_pos += 1;
                        steps += 1;
                        let mut new_p = self.map[new_pos].clone();
                        if new_p.occ != '.' {
                            break;
                        } else {
                            if new_p.room.is_some() {
                                continue;
                            } else {
                                new_p.occ = l;
                                let mut new = self.clone();
                                new.map[new_pos] = new_p;
                                let mut old_p = p.clone();
                                old_p.room = Some(r.clone());
                                new.map[pos] = old_p;
                                let multiplier = match l {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!(),
                                };
                                new.energy += steps * multiplier;
                                moves.push(new)
                            }
                        }
                    }
                }
            } else if p.occ != '.' {
                // not outside room, and occupied
                let target = match p.occ {
                    'A' => 2,
                    'B' => 4,
                    'C' => 6,
                    'D' => 8,
                    _ => unreachable!(),
                };
                let mut target_p = self.map[target].clone();
                let target_room = target_p.room.unwrap();
                if target_room.room.is_empty() || target_room.room.iter().all(|c| *c == p.occ) {
                    // moveable
                    let mut new = self.clone();
                    new.map[pos].occ = '.';
                    // steps to outside of room
                    let mut steps = 0;
                    let delta = (target as i32 - pos as i32).signum();
                    let mut pos = pos;
                    while pos != target {
                        pos = (pos as i32 + delta) as usize;
                        steps += 1;
                        if self.map[pos].occ != '.' {
                            continue 'hall;
                        }
                    }
                    // steps to take inside of room;
                    steps += target_room.size - target_room.room.len();
                    let mut new_room = target_room.room.clone();
                    new_room.push(p.occ);
                    target_p.room = Some(R::new(target_room.size, p.occ, new_room));
                    new.map[target] = target_p;
                    let multiplier = match p.occ {
                        'A' => 1,
                        'B' => 10,
                        'C' => 100,
                        'D' => 1000,
                        _ => unreachable!(),
                    };
                    new.energy += steps * multiplier;
                    moves.push(new);
                } else {
                    // not enterable
                    continue;
                }
            } else {
                continue;
            }
        }
        moves
    }
}

impl Eq for M {}

impl PartialOrd for M {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.energy.partial_cmp(&self.energy)
    }
}

impl Ord for M {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for M {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hall = self.map.clone();
        let mut rooms: Vec<R> = Vec::new();
        for p in &hall {
            write!(f, "{}", p.occ)?;
            if let Some(r) = &p.room {
                rooms.push(r.clone());
            }
        }
        writeln!(f, "        Energy: {}", self.energy)?;
        let mut r1 = [' '; 11];
        let mut r2 = [' '; 11];
        for (i, r) in rooms.into_iter().enumerate() {
            r1[2 + i * 2] = *r.room.get(1).unwrap_or(&'.');
            r2[2 + i * 2] = *r.room.get(0).unwrap_or(&'.');
        }
        for x in r1 {
            write!(f, "{}", x)?;
        }
        write!(f, "\n")?;
        for x in r2 {
            write!(f, "{}", x)?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct P {
    occ: char,
    room: Option<R>,
}

impl P {
    fn new(occ: char, room: Option<R>) -> Self {
        Self { occ, room }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct R {
    size: usize,
    occ: char,
    room: Vec<char>,
}

impl R {
    fn new(size: usize, occ: char, room: Room) -> Self {
        Self { size, occ, room }
    }
}

fn run(m: M) -> M {
    let mut map = m;
    let mut moves: BinaryHeap<M> = BinaryHeap::new();
    moves.extend(map.valid_moves());
    let mut visited: HashSet<M> = HashSet::new();
    loop {
        map = moves.pop().unwrap();
        // println!("{}", map);
        if map.is_done() {
            break map;
        } else {
            if visited.contains(&map) {
                continue;
            } else {
                visited.insert(map.clone());
                moves.extend(map.valid_moves());
            }
        }
    }
}

#[allow(unused_variables)]
pub fn day_23() {
    // let input = get_input(23);
    // test()
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

fn part_one() -> usize {
    // stacks
    let a = vec!['B', 'D'];
    let b = vec!['A', 'C'];
    let c = vec!['A', 'D'];
    let d = vec!['C', 'B'];
    let map = M::new(2, a, b, c, d);
    println!("{}\n\n", map);
    let map = run(map);
    map.energy
}

fn part_two() -> usize {
    // stacks
    let a = vec!['B', 'D', 'D', 'D'];
    let b = vec!['A', 'B', 'C', 'C'];
    let c = vec!['A', 'A', 'B', 'D'];
    let d = vec!['C', 'C', 'A', 'B'];
    let map = M::new(4, a, b, c, d);
    println!("{}\n\n", map);
    let map = run(map);
    map.energy
}

fn test() {
    // stacks
    let a = vec!['A', 'B'];
    let b = vec!['D', 'C'];
    let c = vec!['C', 'B'];
    let d = vec!['A', 'D'];
    let map = M::new(2, a, b, c, d);
    println!("{}\n\n", map);
    let map = run(map);
    println!("Energy: {}", map.energy);
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, BinaryHeap};

    use crate::days::day_23::M;

    #[test]
    fn test_one() {
        let a = vec![];
        let b = vec!['B', 'B'];
        let c = vec!['C', 'C'];
        let d = vec!['D', 'D'];
        let mut map = M::new(2, a, b, c, d);
        map.map[0].occ = 'A';
        map.map[1].occ = 'A';
        let mut moves: BinaryHeap<M> = BinaryHeap::from(map.valid_moves());
        eprintln!("{}", map);
        let map = moves.pop().unwrap();
        eprintln!("{}", map);
        moves.extend(map.valid_moves());
        for m in moves {
            println!("{}", m);
        }
    }

    #[test]
    fn test_two() {
        let a = vec!['B', 'A'];
        let b = vec!['A', 'B'];
        let c = vec!['C', 'C'];
        let d = vec!['D', 'D'];
        let mut map = M::new(2, a, b, c, d);
        let mut moves: BinaryHeap<M> = BinaryHeap::from(map.valid_moves());
        let mut visited = BTreeSet::new();
        eprintln!("{}", map);
        loop {
            map = moves.pop().unwrap();
            if map.is_done() {
                break;
            } else {
                if visited.contains(&map) {
                    continue;
                } else {
                    visited.insert(map.clone());
                    moves.extend(map.valid_moves());
                }
            }
        }
        eprintln!("{}", map);
        // assert!(moves.into_iter().any(|m| m.is_done()))
    }
}
