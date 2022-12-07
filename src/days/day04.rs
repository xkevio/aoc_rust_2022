use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day04.txt");

fn parse_input() -> Vec<(usize, usize, usize, usize)> {
    INPUT
        .lines()
        .flat_map(|l| {
            l.split(&['-', ','][..])
                .map(|a| a.parse::<usize>().unwrap())
                .tuples()
        })
        .collect_vec()
}

pub fn part1() -> usize {
    parse_input()
        .iter()
        .filter(|(a1, b1, a2, b2)| (a2 >= a1 && b2 <= b1) || (a1 >= a2 && b1 <= b2))
        .count()
}

pub fn part2() -> usize {
    parse_input()
        .iter()
        .filter(|(a1, b1, a2, b2)| (a2 >= a1 && a2 <= b1) || (a1 >= a2 && a1 <= b2))
        .count()
}
