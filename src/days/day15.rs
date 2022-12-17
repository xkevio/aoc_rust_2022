use std::ops::RangeInclusive;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../input/day15.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Sensor(i64, i64),
    Beacon(i64, i64),
}

fn parse_input() -> FxHashMap<Node, Node> {
    INPUT.lines().fold(FxHashMap::default(), |mut acc, c| {
        let (sensor, beacon) = c.split_once(':').unwrap();

        let (sx, sy) = scan_fmt!(sensor, "Sensor at x={}, y={}", i64, i64).unwrap();
        let (bx, by) = scan_fmt!(beacon, "closest beacon is at x={}, y={}", i64, i64).unwrap();

        acc.insert(Node::Sensor(sx, sy), Node::Beacon(bx, by));
        acc
    })
}

fn get_ranges_per_row(positions: &FxHashMap<Node, Node>, y: i64) -> FxHashSet<RangeInclusive<i64>> {
    let mut marked_positions = FxHashSet::<RangeInclusive<i64>>::default();

    for (sensor, beacon) in positions {
        if let (Node::Sensor(sx, sy), Node::Beacon(ex, ey)) = (sensor, beacon) {
            let distance = (sx - ex).abs() + (sy - ey).abs();
            let y_distance = sy.abs_diff(y) as i64;

            if y_distance <= distance {
                marked_positions
                    .insert((sx - distance + y_distance)..=(sx + distance - y_distance));
            }
        }
    }

    marked_positions
}

fn check_row(positions: &FxHashMap<Node, Node>, y: i64) -> usize {
    let ranges = get_ranges_per_row(positions, y);

    let min = ranges.iter().min_by_key(|k| k.start()).unwrap().start();
    let max = ranges.iter().max_by_key(|k| k.end()).unwrap().end();

    (max - min) as usize
}

pub fn part1() -> usize {
    check_row(&parse_input(), 2_000_000)
}

pub fn part2() -> i64 {
    let input = parse_input();

    for y in 0..=4_000_000 {
        let ranges = get_ranges_per_row(&input, y);

        let tx = ranges
            .iter()
            .sorted_by(|a, b| a.start().cmp(b.start()))
            .cloned()
            .reduce(|acc, c| {
                if c.end() > acc.end() && c.start() <= acc.end() {
                    *acc.start()..=*c.end()
                } else {
                    acc
                }
            })
            .map(|acc| acc.end() + 1);

        if let Some(x) = tx {
            if (0..=4_000_000).contains(&x) {
                return x * 4_000_000 + y;
            }
        }
    }

    0
}
