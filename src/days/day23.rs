use itertools::Itertools;
use proc_util::RotateEnum;
use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("../../input/day23.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, RotateEnum)]
enum CardinalDirection {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Elf {
    x: i64,
    y: i64,
}

fn parse_input() -> FxHashSet<Elf> {
    INPUT
        .lines()
        .enumerate()
        .fold(FxHashSet::default(), |mut acc, (y, l)| {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    acc.insert(Elf {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }

            acc
        })
}

#[rustfmt::skip]
fn has_neighbor(elf: &Elf, elf_positions: &FxHashSet<Elf>) -> bool {
    for (x, y) in [(0, -1), (-1, 0), (0, 1), (1, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)] {
        if elf_positions.contains(&Elf { x: elf.x + x, y: elf.y + y }) {
            return true;
        }
    }

    false
}

fn check_pos_for_elves(
    elf_positions: &FxHashSet<Elf>,
    current_elf: &Elf,
    positions: &[(i64, i64)],
    new_pos: (i64, i64),
) -> Option<(i64, i64)> {
    for (x, y) in positions {
        if elf_positions.contains(&Elf {
            x: current_elf.x + x,
            y: current_elf.y + y,
        }) {
            return None;
        }
    }

    Some(new_pos)
}

fn check_proposed_position(
    dir: &CardinalDirection,
    map: &FxHashSet<Elf>,
    elf: &Elf,
) -> Option<(i64, i64)> {
    match dir {
        CardinalDirection::North => {
            check_pos_for_elves(map, elf, &[(-1, -1), (0, -1), (1, -1)], (elf.x, elf.y - 1))
        }
        CardinalDirection::East => {
            check_pos_for_elves(map, elf, &[(1, 1), (1, 0), (1, -1)], (elf.x + 1, elf.y))
        }
        CardinalDirection::South => {
            check_pos_for_elves(map, elf, &[(1, 1), (0, 1), (-1, 1)], (elf.x, elf.y + 1))
        }
        CardinalDirection::West => {
            check_pos_for_elves(map, elf, &[(-1, -1), (-1, 0), (-1, 1)], (elf.x - 1, elf.y))
        }
    }
}

fn simulate_round(map: &FxHashSet<Elf>, first_dir: &CardinalDirection) -> FxHashSet<Elf> {
    let mut proposed: FxHashMap<Elf, Option<(i64, i64)>> =
        map.iter().fold(FxHashMap::default(), |mut acc, t| {
            if !has_neighbor(t, map) {
                acc.insert(*t, None);
            } else {
                let mut dir = *first_dir;

                while check_proposed_position(&dir, map, t).is_none() {
                    dir = dir.next();
                    if dir == *first_dir {
                        break;
                    }
                }

                acc.insert(*t, check_proposed_position(&dir, map, t));
            }

            acc
        });

    let mut map = map.clone();
    let duplicates = proposed.values().duplicates().copied().collect_vec();
    proposed.retain(|_, b| !duplicates.contains(b));

    for (k, v) in proposed {
        if let Some((x, y)) = v {
            map.remove(&k);
            map.insert(Elf { x, y });
        }
    }

    map
}

pub fn part1() -> usize {
    let mut map = parse_input();
    let mut dir = CardinalDirection::North;

    for _ in 0..10 {
        map = simulate_round(&map, &dir);
        dir = dir.next();
    }

    let x_len =
        map.iter().max_by_key(|t| t.x).unwrap().x - map.iter().min_by_key(|t| t.x).unwrap().x + 1;
    let y_len =
        map.iter().max_by_key(|t| t.y).unwrap().y - map.iter().min_by_key(|t| t.y).unwrap().y + 1;

    (x_len as usize * y_len as usize) - map.len()
}

pub fn part2() -> usize {
    let mut map = parse_input();
    let mut dir = CardinalDirection::North;

    let mut counter = 1;
    loop {
        let elves = map.iter().collect_vec();
        let new_map = simulate_round(&map, &dir);

        if elves == new_map.iter().collect_vec() {
            break;
        }

        map = new_map;
        dir = dir.next();

        counter += 1;
    }

    counter
}
