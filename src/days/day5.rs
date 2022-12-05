use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../input/day5.txt");
type Stacks = Vec<Vec<char>>;

fn parse_input() -> Stacks {
    INPUT
        .split_once("\n\n")
        .map(|(a, _)| {
            let mut stacks = Vec::new();

            let num_stacks = a.lines().last().map(|l| l.len() / 4 + 1).unwrap();
            stacks.resize(num_stacks, Vec::new());

            for l in a.lines() {
                let letter_pos = l
                    .char_indices()
                    .filter(|(_, ch)| ch.is_ascii_alphabetic())
                    .collect_vec();

                for (pos, letter) in letter_pos {
                    stacks[pos / 4].push(letter);
                }
            }

            stacks
        })
        .unwrap()
}

fn apply_instructions(stacks: &mut Stacks, lifo: bool) {
    if let Some((_, b)) = INPUT.split_once("\n\n") {
        for l in b.lines() {
            let (amount, from, to) =
                scan_fmt!(l, "move {} from {} to {}", usize, usize, usize).unwrap();

            for i in (0..amount).rev() {
                let crate_element = if lifo {
                    stacks[from - 1].remove(0)
                } else {
                    stacks[from - 1].remove(i)
                };

                stacks[to - 1].insert(0, crate_element);
            }
        }
    };
}

fn stack_to_string(stacks: &Stacks) -> String {
    stacks.iter().flat_map(|v| v.first()).collect()
}

pub fn part1() -> String {
    let mut stacks = parse_input();

    apply_instructions(&mut stacks, true);
    stack_to_string(&stacks)
}

pub fn part2() -> String {
    let mut stacks = parse_input();

    apply_instructions(&mut stacks, false);
    stack_to_string(&stacks)
}
