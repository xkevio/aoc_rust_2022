use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day08.txt");

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec()
}

fn viewing_distance(x: u32, trees: &[u32]) -> usize {
    trees
        .iter()
        .position(|&a| a >= x)
        .map_or(trees.len(), |a| a + 1)
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
            let e = th[r][c];

            let left = viewing_distance(e, &th[r][0..c].iter().rev().copied().collect_vec());
            let right = viewing_distance(e, &th[r][c + 1..th[0].len()]);
            let up = viewing_distance(e, &th[0..r].iter().rev().map(|a| a[c]).collect_vec());
            let down = viewing_distance(e, &th[r + 1..th.len()].iter().map(|a| a[c]).collect_vec());

            if left * right * up * down > scenic_score {
                scenic_score = left * right * up * down;
            }
        }
    }

    scenic_score
}
