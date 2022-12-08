use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day08.txt");

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec()
}

pub fn part1() -> usize {
    let th = parse_input();
    let mut vis_score = 0;

    for r in 0..th.len() {
        for c in 0..th[0].len() {
            let cur = th[r][c];

            if th[r][0..c].iter().all(|&e| e < cur)
                || th[r][c + 1..th[0].len()].iter().all(|&e| e < cur)
                || th[0..r].iter().all(|e| e[c] < cur)
                || th[r + 1..th.len()].iter().all(|e| e[c] < cur)
            {
                vis_score += 1;
            }
        }
    }

    vis_score
}

pub fn part2() -> usize {
    let th = parse_input();
    let mut scenic_score = 0;

    for r in 1..th.len() - 1 {
        for c in 1..th[0].len() - 1 {
            let cur_element = th[r][c];

            let left = th[r][0..c]
                .iter()
                .rev()
                .position(|a| *a >= cur_element)
                .unwrap_or(th[r][0..c].len() - 1)
                + 1;
            let right = th[r][c + 1..th[0].len()]
                .iter()
                .position(|a| *a >= cur_element)
                .unwrap_or(th[r][c + 1..th[0].len()].len() - 1)
                + 1;
            let up = th[0..r]
                .iter()
                .rev()
                .position(|a| a[c] >= cur_element)
                .unwrap_or(th[0..r].len() - 1)
                + 1;
            let down = th[r + 1..th.len()]
                .iter()
                .position(|a| a[c] >= cur_element)
                .unwrap_or(th[r + 1..th.len()].len() - 1)
                + 1;

            if left * right * up * down > scenic_score {
                scenic_score = left * right * up * down;
            }
        }
    }

    scenic_score
}
