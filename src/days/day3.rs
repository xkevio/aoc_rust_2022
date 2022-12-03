use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day3.txt");

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
        .tuples()
        .map(|(a, b, c)| {
            let a_set: HashSet<char> = HashSet::from_iter(a.chars());
            let b_set: HashSet<char> = HashSet::from_iter(b.chars());
            let c_set: HashSet<char> = HashSet::from_iter(c.chars());

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
