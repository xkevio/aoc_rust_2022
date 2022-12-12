use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../input/day11.txt");

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u128>,
    op: (char, String),
    divisor: u128,
    monkeys: (usize, usize),
    inspection_count: u128,
}

impl Monkey {
    // returns the monkey thrown to and worry level
    pub fn turn(&mut self, worry_div: u128, mult: u128) -> Vec<(usize, u128)> {
        self.starting_items
            .iter()
            .map(|item| {
                self.inspection_count += 1;

                let new_worry_level = match self.op.clone() {
                    ('*', v) => item * v.parse::<u128>().unwrap_or(*item),
                    ('+', v) => item + v.parse::<u128>().unwrap_or(*item),
                    _ => unreachable!(),
                } / worry_div;

                if new_worry_level % self.divisor == 0 {
                    (self.monkeys.0, new_worry_level % mult)
                } else {
                    (self.monkeys.1, new_worry_level % mult)
                }
            })
            .collect()
    }
}

fn parse_input() -> Vec<Monkey> {
    INPUT
        .split("\n\n")
        .map(|m| {
            let lines = m.lines().skip(1).collect_vec();

            let starting_items = lines[0]
                .split(&[':', ','][..])
                .filter_map(|f| f.trim().parse::<u128>().ok())
                .collect_vec();
            let op = scan_fmt!(lines[1], "Operation: new = old {} {}", char, String).unwrap();
            let divisor = scan_fmt!(lines[2], "Test: divisible by {}", u128).unwrap();

            let true_monkey = scan_fmt!(lines[3], "If true: throw to monkey {}", usize).unwrap();
            let false_monkey = scan_fmt!(lines[4], "If false: throw to monkey {}", usize).unwrap();

            let monkeys = (true_monkey, false_monkey);

            Monkey {
                starting_items,
                op,
                divisor,
                monkeys,
                inspection_count: 0,
            }
        })
        .collect()
}

fn solve(iter: usize, worry_div: u128) -> u128 {
    let mut monkeys = parse_input();
    let mult = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..iter {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let v = monkey.turn(worry_div, mult);
            monkey.starting_items.clear();

            for (other, w) in v {
                monkeys[other as usize].starting_items.push(w);
            }
        }
    }

    monkeys
        .iter()
        .sorted_by(|a, b| b.inspection_count.cmp(&a.inspection_count))
        .take(2)
        .map(|m| m.inspection_count)
        .product()
}

pub fn part1() -> u128 {
    solve(20, 3)
}

pub fn part2() -> u128 {
    solve(10_000, 1)
}
