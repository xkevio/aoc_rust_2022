#![allow(dead_code)]

use colored::Colorize;
use std::time::Instant;

mod days;

#[rustfmt::skip]
fn main() {
    println!("{}", "\n🎄 Advent of Code 2022 🎄\n".green().bold());
    let time = Instant::now();
    
    println!("{} {}", "Part 1:".yellow().bold(), days::day21::part1());
    let part1_time = time.elapsed();

    println!("{} {}", "Part 2:".yellow().bold(), days::day21::part2());
    let part2_time = time.elapsed();

    println!("------------");
    println!("{} took {}µs", "Part 1".yellow().bold(), part1_time.as_micros());
    println!("{} took {}µs", "Part 2".yellow().bold(), part2_time.as_micros());
}
