use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day20.txt");

fn parse_input() -> Vec<i64> {
    INPUT.lines().filter_map(|l| l.parse().ok()).collect()
}

fn mix(input: &[i64], times: usize, encryption_key: usize) -> i64 {
    let mut new_order = input
        .iter()
        .enumerate()
        .map(|(i, v)| (i, *v * encryption_key as i64))
        .collect_vec();

    let mut sum = 0;

    for _ in 0..times {
        for (ind, v) in input.iter().enumerate() {
            let v = v * encryption_key as i64;

            if v == 0 {
                continue;
            }

            let pos = new_order
                .iter()
                .position(|(i, a)| *a == v && *i == ind)
                .unwrap();

            let new_index = (pos as i64 + v).rem_euclid(input.len() as i64 - 1);

            new_order.remove(pos);
            new_order.insert(new_index as usize, (ind, v));
        }

        let zero = new_order.iter().position(|(_, a)| *a == 0).unwrap();
        sum = new_order[(zero + 1000) % input.len()].1
            + new_order[(zero + 2000) % input.len()].1
            + new_order[(zero + 3000) % input.len()].1;
    }

    sum
}

pub fn part1() -> i64 {
    mix(&parse_input(), 1, 1)
}

pub fn part2() -> i64 {
    mix(&parse_input(), 10, 811589153)
}
