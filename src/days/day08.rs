use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day08.txt");

fn parse_input() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec()
}

fn view_score<'a, I: IntoIterator<Item = &'a u32>>(x: u32, trees: I, size: usize) -> usize {
    trees
        .into_iter()
        .position(|&a| a >= x)
        .map_or(size, |a| a + 1)
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

#[rustfmt::skip]
pub fn part2() -> usize {
    let th = parse_input();
    let mut scenic_score = 0;

    for r in 1..th.len() - 1 {
        for c in 1..th[0].len() - 1 {
            let e = th[r][c];

            let left = view_score(e, th[r][0..c].iter().rev(), c);
            let right = view_score(e, &th[r][c + 1..th[0].len()], th[0].len() - (c + 1));
            let up = view_score(e, th[0..r].iter().map(|a| &a[c]).rev(), r);
            let down = view_score(e, th[r + 1..th.len()].iter().map(|a| &a[c]), th.len() - (r + 1));

            if left * right * up * down > scenic_score {
                scenic_score = left * right * up * down;
            }
        }
    }

    scenic_score
}
