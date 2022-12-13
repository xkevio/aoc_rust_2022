use colored::Colorize;

const INPUT: &str = include_str!("../../input/day13.txt");

fn parse_line(line: &str) -> Vec<Vec<u32>> {
    let mut bracket_stack = Vec::<char>::new();
    let mut num_stack = Vec::<u32>::new();

    let mut list: Vec<Vec<u32>> = Vec::new();

    line.chars().for_each(|c| match c {
        '[' => bracket_stack.push(c),
        ']' => {
            bracket_stack.pop();
            list.push(num_stack.clone());
            num_stack.clear();
        }
        _ => {
            if let Some(v) = c.to_digit(10) {
                num_stack.push(v);
            }
        }
    });

    list
}

fn check_order() -> usize {
    let mut count = 0;

    for (index, pair) in INPUT.split("\n\n").enumerate() {
        let (first, second) = pair.split_once("\n").unwrap();

        let first_list = parse_line(first);
        let second_list = parse_line(second);

        println!("{}", "FIRST LIST START".red().bold());
        println!("{:?}", &first_list);
        println!("{}", "FIRST LIST END\n".red().bold());

        println!("{}", "SECOND LIST START".green().bold());
        println!("{:?}", &second_list);
        println!("{}", "SECOND LIST END\n".green().bold());

        if first_list.iter().all(|v| v.is_empty()) && second_list.iter().all(|v| v.is_empty()) {
            println!("both lists only consist of empty lists!");
            if first_list.len() < second_list.len() {
                println!("-> correct order, left list (empty) ran out of elements first!");
                count += index + 1;
            } else {
                println!("-> wrong order or continue");
            }
            continue;
        } 

        'outer: for (a, b) in first_list.iter().zip(second_list.iter()) {
            for i in 0..(a.len().max(b.len())) {
                let first_el = a.get(i);
                let second_el = b.get(i);

                if first_el.is_none() && second_el.is_some() {
                    count += index + 1;
                    println!("-> correct order!, left list ran out of elements first\n");
                    break 'outer;
                }

                if first_el.is_some() && second_el.is_none() {
                    println!("-> wrong order!, right list ran out of elements first\n");
                    break 'outer;
                }

                if first_el.unwrap() < second_el.unwrap() {
                    count += index + 1;
                    println!("-> correct order!, left element smaller than right element\n");
                    break 'outer;
                }

                if first_el.unwrap() > second_el.unwrap() {
                    println!("-> wrong order!, left element bigger than right element\n");
                    break 'outer;
                }
            }
        }
    }

    count
}

pub fn part1() -> usize {
    check_order()
}

pub fn part2() -> usize {
    0
}
