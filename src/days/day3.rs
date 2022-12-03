use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day3.txt");

#[inline(always)]
fn letter_to_prio(ch: char) -> usize {
    if ch.is_ascii_uppercase() {
        (27 + (ch as u8 - b'A')) as usize
    } else {
        (1 + (ch as u8 - b'a')) as usize
    }
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            a.chars()
                .filter(|ch| b.contains(*ch))
                .dedup()
                .map(letter_to_prio)
                .sum::<usize>()
        })
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .filter(|ch| b.contains(*ch) && c.contains(*ch))
                .dedup()
                .map(letter_to_prio)
                .sum::<usize>()
        })
        .sum()
}
