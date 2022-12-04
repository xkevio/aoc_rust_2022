use itertools::Itertools;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../input/day4.txt");

fn parse_input() -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    INPUT
        .lines()
        .flat_map(|l| {
            l.split_once(',').map(|(a, b)| {
                let a_range = a
                    .split_once('-')
                    .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
                    .unwrap();

                let b_range = b
                    .split_once('-')
                    .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
                    .unwrap();

                (a_range, b_range)
            })
        })
        .collect_vec()
}

pub fn part1() -> usize {
    parse_input()
        .iter()
        .map(|(a, b)| {
            usize::from(
                (b.start() >= a.start() && b.end() <= a.end())
                    || (a.start() >= b.start() && a.end() <= b.end()),
            )
        })
        .sum()
}

pub fn part2() -> usize {
    parse_input()
        .iter()
        .map(|(a, b)| {
            usize::from(
                (b.start() >= a.start() && b.start() <= a.end())
                    || (a.start() >= b.start() && a.start() <= b.end()),
            )
        })
        .sum()
}
