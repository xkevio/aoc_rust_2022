use serde::Deserialize;

const INPUT: &str = include_str!("../../input/day13.txt");

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Packet {
    Num(usize),
    Nested(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Packet::Num(a) => match other {
                Packet::Num(b) => a.cmp(b),
                Packet::Nested(_) => other.cmp(self).reverse(),
            },
            Packet::Nested(v) => match other {
                Packet::Num(_) => v.cmp(&vec![(*other).clone()]),
                Packet::Nested(w) => v.cmp(w),
            },
        }
    }
}

fn parse_input() -> Vec<Packet> {
    INPUT
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

fn solve() -> (usize, usize) {
    let packets = parse_input();

    let part1 = check_order(&packets);
    let part2 = find_decoder_key(&packets);

    (part1, part2)
}

fn check_order(packets: &[Packet]) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, a)| a[0] < a[1])
        .fold(0, |acc, (pos, _)| acc + pos + 1)
}

fn find_decoder_key(packets: &[Packet]) -> usize {
    let mut extra_packets = packets.to_vec();

    let packet_two = Packet::Nested(vec![Packet::Nested(vec![Packet::Num(2)])]);
    let packet_six = Packet::Nested(vec![Packet::Nested(vec![Packet::Num(6)])]);

    extra_packets.push(packet_two.clone());
    extra_packets.push(packet_six.clone());
    extra_packets.sort();

    let key_one = extra_packets.iter().position(|v| *v == packet_two).unwrap() + 1;
    let key_two = extra_packets.iter().position(|v| *v == packet_six).unwrap() + 1;

    key_one * key_two
}

pub fn part1() -> usize {
    solve().0
}

pub fn part2() -> usize {
    solve().1
}
