use itertools::Itertools;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../input/day07.txt");

fn parse_input() -> BTreeMap<String, usize> {
    let mut file_system = BTreeMap::<String, usize>::new();
    let mut directory_stack = Vec::<String>::new();

    for line in INPUT.lines() {
        match &line.split_whitespace().collect_vec()[..] {
            ["$", "cd", cd] if *cd != ".." => {
                let path_name = match directory_stack.last() {
                    Some(last) => format!("{}{}", last, cd),
                    None => (*cd).into(),
                };

                directory_stack.push(path_name.clone());
                file_system.entry(path_name).or_default();
            }
            ["$", "cd", cd] if *cd == ".." => {
                directory_stack.pop();
            }
            [value, _] if value.parse::<usize>().is_ok() => {
                for prev_dir in &directory_stack {
                    file_system
                        .entry(prev_dir.clone())
                        .and_modify(|v| *v += value.parse::<usize>().unwrap());
                }
            }
            _ => {}
        }
    }

    file_system
}

pub fn part1() -> usize {
    parse_input().values().filter(|&&v| v <= 100_000).sum()
}

pub fn part2() -> usize {
    *parse_input()
        .values()
        .filter(|&&v| match parse_input().get("/") {
            Some(max) => ((70_000_000 - max) + v) >= 30_000_000,
            None => false,
        })
        .min()
        .unwrap()
}