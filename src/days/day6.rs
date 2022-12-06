use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day6.txt");

fn get_processed_amount(amount: usize) -> usize {
    INPUT
        .chars()
        .collect_vec()
        .windows(amount)
        .position(|w| w.iter().all_unique())
        .map(|i| i + amount)
        .unwrap()
}

pub fn part1() -> usize {
    get_processed_amount(4)
}

pub fn part2() -> usize {
    get_processed_amount(14)
}
