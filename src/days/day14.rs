use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day14.txt");

type Pos = (usize, usize);

fn parse_input() -> HashSet<Pos> {
    let mut rocks = HashSet::new();

    for l in INPUT.lines() {
        for (p1, p2) in l.split(" -> ").tuple_windows() {
            let (fx, fy) = p1.split_once(',').unwrap();
            let (sx, sy) = p2.split_once(',').unwrap();

            let first_pair = (fx.parse::<usize>().unwrap(), fy.parse::<usize>().unwrap());
            let second_pair = (sx.parse::<usize>().unwrap(), sy.parse::<usize>().unwrap());

            for i in (first_pair.0.min(second_pair.0))..(second_pair.0.max(first_pair.0)) {
                rocks.insert((i, first_pair.1));
            }

            for i in (first_pair.1.min(second_pair.1))..(second_pair.1.max(first_pair.1)) {
                rocks.insert((first_pair.0, i));
            }

            rocks.insert(first_pair);
            rocks.insert(second_pair);
        }
    }

    rocks
}

// returns free pos if there is one, None if its not able to move any more
fn check_sand_surroundings(positions: &HashSet<Pos>, sand_pos: &Pos) -> Option<Pos> {
    for offset in [(0, 1), (-1, 1), (1, 1)] {
        let (row, col) = (sand_pos.0 as i32 + offset.0, sand_pos.1 as i32 + offset.1);
        if !positions.contains(&(row as usize, col as usize)) {
            return Some((row as usize, col as usize));
        }
    }

    None
}

fn tick(positions: &HashSet<Pos>, sand_pos: &Pos) -> Pos {
    match check_sand_surroundings(positions, sand_pos) {
        Some(pos) => pos,
        None => *sand_pos,
    }
}

fn solve(blockage: bool) -> usize {
    let mut positions = parse_input();
    let abyss = *positions.iter().max_by_key(|k| k.1).unwrap();

    if blockage {
        for x in 0..1000 {
            positions.insert((x, abyss.1 + 2));
        }
    }

    let mut count = 0;
    'a: loop {
        let mut sand_pos = (500, 0);
        let mut new_pos = tick(&positions, &sand_pos);

        if blockage && new_pos == (500, 0) {
            count += 1;
            break;
        }

        if new_pos.1 >= abyss.1 && !blockage {
            break;
        }

        while sand_pos != new_pos {
            sand_pos = new_pos;
            new_pos = tick(&positions, &sand_pos);

            if sand_pos.1 >= abyss.1 && !blockage {
                break 'a;
            }
        }

        count += 1;
        positions.insert(sand_pos);
    }

    count
}

pub fn part1() -> usize {
    solve(false)
}

pub fn part2() -> usize {
    solve(true)
}
