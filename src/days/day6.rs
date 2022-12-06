use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day6.txt");

fn get_processed_amount(amount: usize) -> usize {
    INPUT
        .char_indices()
        .collect_vec()
        .windows(amount)
        .find_map(|t| {
            if t.iter().unique_by(|(_, c)| c).count() == amount {
                Some(t.last().unwrap().0 + 1)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part1() -> usize {
    get_processed_amount(4)
}

pub fn part2() -> usize {
    get_processed_amount(14)
}
