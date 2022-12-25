const INPUT: &str = include_str!("../../input/day25.txt");

fn sum_snafu_nums() -> i64 {
    INPUT
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(i, c)| {
                    let i = l.len() - i - 1;
                    match c {
                        '2' => 2 * 5i64.pow(i as u32),
                        '1' => 5i64.pow(i as u32),
                        '-' => -5i64.pow(i as u32),
                        '=' => -2 * 5i64.pow(i as u32),
                        _ => 0,
                    }
                })
                .sum::<i64>()
        })
        .sum()
}

fn dec_to_snafu(num: i64) -> String {
    let mut digit = num;
    let mut snafu_num = String::new();

    while digit != 0 {
        let (result, remainder) = (digit / 5, digit % 5);
        let carry = i64::from(remainder == 3 || remainder == 4);

        match remainder {
            0 => snafu_num.push('0'),
            1 => snafu_num.push('1'),
            2 => snafu_num.push('2'),
            3 => snafu_num.push('='),
            4 => snafu_num.push('-'),
            _ => unreachable!(),
        }

        digit = result + carry;
    }

    snafu_num.chars().rev().collect()
}

pub fn part1() -> String {
    dec_to_snafu(sum_snafu_nums())
}
