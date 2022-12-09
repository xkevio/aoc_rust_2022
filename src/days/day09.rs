use scan_fmt::scan_fmt;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day09.txt");

fn move_tail(tail: &mut [(i32, i32)]) {
    for i in 1..tail.len() {
        let x_diff = tail[i - 1].0 - tail[i].0;
        let y_diff = tail[i - 1].1 - tail[i].1;

        let pair = if x_diff.abs() >= 2 && y_diff.abs() >= 2 {
            (x_diff - x_diff.signum(), y_diff - y_diff.signum())
        } else if x_diff.abs() >= 2 {
            (x_diff - x_diff.signum(), y_diff)
        } else if y_diff.abs() >= 2 {
            (x_diff, y_diff - y_diff.signum())
        } else {
            (0, 0)
        };

        tail[i] = (tail[i].0 + pair.0, tail[i].1 + pair.1);
    }
}

fn move_rope(
    knots: &[(i32, i32)],
    amount: usize,
    dir: char,
    seen_pos: &mut HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut new_knots = Vec::from(knots);
    match dir {
        'D' => {
            for _ in 0..amount {
                new_knots[0].1 -= 1;
                move_tail(&mut new_knots);
                seen_pos.insert(new_knots[new_knots.len() - 1]);
            }
        }
        'U' => {
            for _ in 0..amount {
                new_knots[0].1 += 1;
                move_tail(&mut new_knots);
                seen_pos.insert(new_knots[new_knots.len() - 1]);
            }
        }
        'R' => {
            for _ in 0..amount {
                new_knots[0].0 += 1;
                move_tail(&mut new_knots);
                seen_pos.insert(new_knots[new_knots.len() - 1]);
            }
        }
        'L' => {
            for _ in 0..amount {
                new_knots[0].0 -= 1;
                move_tail(&mut new_knots);
                seen_pos.insert(new_knots[new_knots.len() - 1]);
            }
        }
        _ => unreachable!(),
    }

    new_knots
}

fn solve_for_n(n: usize) -> usize {
    let mut positions = vec![(0, 0); n];
    let mut seen_pos = HashSet::from([(0, 0)]);

    for line in INPUT.lines() {
        let (dir, amount) = scan_fmt!(line, "{} {}", char, usize).unwrap();
        positions = move_rope(&positions, amount, dir, &mut seen_pos);
    }

    seen_pos.len()
}

pub fn part1() -> usize {
    solve_for_n(2)
}

pub fn part2() -> usize {
    solve_for_n(10)
}
