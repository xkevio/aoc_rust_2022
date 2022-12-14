use itertools::Itertools;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../../input/day14.txt");

type Pos = (usize, usize);

fn parse_input() -> BTreeSet<Pos> {
    let mut rocks = BTreeSet::new();
    for l in INPUT.lines() {
        for (p1, p2) in l.split(" -> ").tuple_windows() {
            let pair1 = p1.split_once(',').unwrap();
            let pair2 = p2.split_once(',').unwrap();

            let parsed_pair1 = (
                pair1.0.parse::<usize>().unwrap(),
                pair1.1.parse::<usize>().unwrap(),
            );

            let parsed_pair2 = (
                pair2.0.parse::<usize>().unwrap(),
                pair2.1.parse::<usize>().unwrap(),
            );

            for i in (parsed_pair1.0.min(parsed_pair2.0))..(parsed_pair2.0.max(parsed_pair1.0)) {
                rocks.insert((i, parsed_pair1.1));
            }

            for i in (parsed_pair1.1.min(parsed_pair2.1))..(parsed_pair2.1.max(parsed_pair1.1)) {
                rocks.insert((parsed_pair1.0, i));
            }

            rocks.insert(parsed_pair1);
            rocks.insert(parsed_pair2);
        }
    }

    rocks
}

// returns free pos if there is one, None if its not able to move any more
fn check_sand_surroundings(positions: &BTreeSet<Pos>, sand_pos: &Pos) -> Option<Pos> {
    for offset in [(0, 1), (-1, 1), (1, 1)] {
        let (row, col) = (sand_pos.0 as i32 + offset.0, sand_pos.1 as i32 + offset.1);
        if !positions.contains(&(row as usize, col as usize)) {
            return Some((row as usize, col as usize));
        }
    }

    None
}

fn tick(positions: &BTreeSet<Pos>, sand_pos: &Pos) -> Pos {
    match check_sand_surroundings(positions, sand_pos) {
        Some(pos) => pos,
        None => *sand_pos,
    }
}

fn solve() -> (usize, usize) {
    let mut positions = parse_input();
    let abyss = *positions.iter().max_by_key(|k| k.1).unwrap();

    let mut count = 0;
    'a: loop {
        let mut sand_pos = (500, 0);
        let mut new_pos = tick(&positions, &sand_pos);

        if new_pos.1 >= abyss.1 {
            break;
        }

        while sand_pos != new_pos {
            sand_pos = new_pos;
            new_pos = tick(&positions, &sand_pos);

            if sand_pos.1 >= abyss.1 {
                break 'a;
            }
        }

        count += 1;
        positions.insert(sand_pos);
    }

    (count, 0)
}

pub fn part1() -> usize {
    solve().0
}

pub fn part2() -> usize {
    0
}
