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
