use std::time;

pub fn day_24() {
    // println!("Part 1: {:?}", part_one());
    println!("Part 2: {:?}", part_two());
}

fn run_block(z: i64, w: i64, a: i64, b: i64, c: i64) -> i64 {
    let x = if ((z % 26) + b) != w { 1 } else { 0 };
    ((z / a) * ((25 * x) + 1)) + ((w + c) * x)
}

// runtime: 10 hours and twenty minutes
fn part_one() -> [i64; 14] {
    let r = (1..=9).rev();
    let t = time::SystemTime::now();
    // 1
    for a in r.clone() {
        // 2
        for b in r.clone() {
            println!("{}, {}", b, t.elapsed().unwrap().as_secs());
            // 3
            for c in r.clone() {
                // 4
                for d in r.clone() {
                    // 5
                    for e in r.clone() {
                        // 6
                        for f in r.clone() {
                            // 7
                            for g in r.clone() {
                                // 8
                                for h in r.clone() {
                                    // 9
                                    for i in r.clone() {
                                        // 10
                                        for j in r.clone() {
                                            // 11
                                            for k in r.clone() {
                                                // 12
                                                for l in r.clone() {
                                                    // 13
                                                    for m in r.clone() {
                                                        // 14
                                                        for n in r.clone() {
                                                            let z = run_block(0, a, 1, 14, 12);
                                                            let z = run_block(z, b, 1, 15, 7);
                                                            let z = run_block(z, c, 1, 12, 1);
                                                            let z = run_block(z, d, 1, 11, 2);
                                                            let z = run_block(z, e, 26, -5, 4);
                                                            let z = run_block(z, f, 1, 14, 15);
                                                            let z = run_block(z, g, 1, 15, 11);
                                                            let z = run_block(z, h, 26, -13, 5);
                                                            let z = run_block(z, i, 26, -16, 3);
                                                            let z = run_block(z, j, 26, -8, 9);
                                                            let z = run_block(z, k, 1, 15, 2);
                                                            let z = run_block(z, l, 26, -8, 3);
                                                            let z = run_block(z, m, 26, 0, 3);
                                                            let z = run_block(z, n, 26, -4, 11);
                                                            if z == 0 {
                                                                return [
                                                                    a, b, c, d, e, f, g, h, i, j,
                                                                    k, l, m, n,
                                                                ];
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

// runtime: 4 minutes
fn part_two() -> [i64; 14] {
    let r = 1..=9;
    let t = time::SystemTime::now();
    // 1
    for a in r.clone() {
        // 2
        for b in r.clone() {
            println!("{}, {}", b, t.elapsed().unwrap().as_secs());
            // 3
            for c in r.clone() {
                // 4
                for d in r.clone() {
                    // 5
                    for e in r.clone() {
                        // 6
                        for f in r.clone() {
                            // 7
                            for g in r.clone() {
                                // 8
                                for h in r.clone() {
                                    // 9
                                    for i in r.clone() {
                                        // 10
                                        for j in r.clone() {
                                            // 11
                                            for k in r.clone() {
                                                // 12
                                                for l in r.clone() {
                                                    // 13
                                                    for m in r.clone() {
                                                        // 14
                                                        for n in r.clone() {
                                                            let z = run_block(0, a, 1, 14, 12);
                                                            let z = run_block(z, b, 1, 15, 7);
                                                            let z = run_block(z, c, 1, 12, 1);
                                                            let z = run_block(z, d, 1, 11, 2);
                                                            let z = run_block(z, e, 26, -5, 4);
                                                            let z = run_block(z, f, 1, 14, 15);
                                                            let z = run_block(z, g, 1, 15, 11);
                                                            let z = run_block(z, h, 26, -13, 5);
                                                            let z = run_block(z, i, 26, -16, 3);
                                                            let z = run_block(z, j, 26, -8, 9);
                                                            let z = run_block(z, k, 1, 15, 2);
                                                            let z = run_block(z, l, 26, -8, 3);
                                                            let z = run_block(z, m, 26, 0, 3);
                                                            let z = run_block(z, n, 26, -4, 11);
                                                            if z == 0 {
                                                                return [
                                                                    a, b, c, d, e, f, g, h, i, j,
                                                                    k, l, m, n,
                                                                ];
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}
