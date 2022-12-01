use std::time::Instant;

use colored::Colorize;

mod days;

fn main() {
    println!("{}", "\n🎄 Advent of Code 2022 🎄\n".green().bold());
    let time = Instant::now();
    
    println!("{} {}", "Part 1:".yellow().bold(), "");
    let part1_time = time.elapsed();

    println!("{} {}", "Part 2:".yellow().bold(), "");
    let part2_time = time.elapsed();

    println!("------------");
    println!("{} took {}ms", "Part 1".yellow().bold(), part1_time.as_millis());
    println!("{} took {}ms", "Part 2".yellow().bold(), part2_time.as_millis());
}
