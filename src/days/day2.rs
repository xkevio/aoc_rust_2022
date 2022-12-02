const INPUT: &str = include_str!("../../input/day2.txt");

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
                + match (you, opp) {
                    ("X", "A") => 3, // 23 -> 3
                    ("X", "B") => 1,
                    // X C 2
                    ("Y", "A") => 1, // 24 -> 1
                    // Y B 2
                    ("Y", "C") => 3,
                    // Z A 2,           25 -> 2
                    ("Z", "B") => 3,
                    ("Z", "C") => 1,
                    _ => 2,
                } as usize
        })
        .sum::<usize>()
}
