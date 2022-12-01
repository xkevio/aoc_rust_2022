use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day1.txt");

fn parse_input() -> Vec<usize> {
    INPUT
        .split("\n\n")
        .map(|cal| cal.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .collect()
}

pub fn part1() -> usize {
    parse_input()[0]
}

pub fn part2() -> usize {
    parse_input().iter().take(3).sum()
}
