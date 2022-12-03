use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day3.txt");

// a.chars()
//     .cartesian_product(b.chars())
//     .filter(|(a, b)| a.eq(b))
//     .dedup()
//     .map(|(a, _)| {
//         if a.is_ascii_uppercase() {
//             (27 + (a as u8 - b'A')) as usize
//         } else {
//             (1 + (a as u8 - b'a')) as usize
//         }
//     })
//     .sum::<usize>()
pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);

            let a_set: HashSet<char> = HashSet::from_iter(a.chars());
            let b_set: HashSet<char> = HashSet::from_iter(b.chars());

            a_set
                .intersection(&b_set)
                .map(|&a| {
                    if a.is_ascii_uppercase() {
                        (27 + (a as u8 - b'A')) as usize
                    } else {
                        (1 + (a as u8 - b'a')) as usize
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut c| {
            let a_set: HashSet<char> = HashSet::from_iter(c.next().unwrap().chars());
            let b_set: HashSet<char> = HashSet::from_iter(c.next().unwrap().chars());
            let c_set: HashSet<char> = HashSet::from_iter(c.next().unwrap().chars());

            a_set
                .intersection(&b_set)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&c_set)
                .map(|&a| {
                    if a.is_ascii_uppercase() {
                        (27 + (a as u8 - b'A')) as usize
                    } else {
                        (1 + (a as u8 - b'a')) as usize
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
