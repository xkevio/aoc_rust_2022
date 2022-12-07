const INPUT: &str = include_str!("../../input/day02.txt");

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let (opp, you) = l.split_once(' ').unwrap();
            (you.as_bytes()[0] - b'W') as usize
                + (((you.as_bytes()[0] - opp.as_bytes()[0]) % 3 + 2) % 3 * 3) as usize
        })
        .sum::<usize>()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let (opp, you) = l.split_once(' ').unwrap();
            ((you.as_bytes()[0] % b'X') * 3) as usize
                + (((opp.as_bytes()[0] - b'A') as i16 + (you.as_bytes()[0] as i16 - b'Y' as i16))
                    .rem_euclid(3)
                    + 1) as usize
        })
        .sum::<usize>()
}
