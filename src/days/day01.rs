const INPUT: &str = include_str!("../../input/day01.txt");

/*

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day01.txt");

fn main() {
    let input: Vec<usize> = INPUT
        .split("\n\n")
        .map(|cal| cal.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .collect();

    println!("Part 1: {}", input[0]);
    println!("Part 2: {}", input[0] + input[1] + input[2]);
}

*/

fn parse_input() -> Vec<usize> {
    INPUT
        .split("\n\n")
        .map(|cal| cal.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

pub fn part1() -> usize {
    *parse_input().iter().max().unwrap()
}

pub fn part2() -> usize {
    let mut sorted_input = parse_input();
    sorted_input.sort_by(|a, b| b.cmp(a));

    sorted_input.iter().take(3).sum()
}
