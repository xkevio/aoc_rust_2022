const INPUT: &str = include_str!("../../input/day10.txt");

type Screen = Vec<Vec<char>>;

fn check_cycle(cy_count: i32, x: i32) -> i32 {
    if (cy_count - 20) % 40 == 0 {
        cy_count * x
    } else {
        0
    }
}

fn draw_sprite_on_screen(current_image: &Screen, x: i32, cy_count: i32) -> Screen {
    let mut screen = current_image.clone();

    let row = (cy_count - 1) / 40;
    let col = (cy_count - 1) % 40;

    if x == col || x - 1 == col || x + 1 == col {
        screen[row as usize][col as usize] = '#';
    }

    screen
}

fn solve() -> (i32, Screen) {
    let mut screen = vec![vec!['.'; 40]; 6];
    let (_, signal, _) = INPUT.lines().fold((0, 0, 1), |mut acc, el| {
        let (ins, val) = el.split_at(4);

        acc.0 += 1;
        acc.1 += check_cycle(acc.0, acc.2);
        screen = draw_sprite_on_screen(&screen, acc.2, acc.0);

        if ins == "addx" {
            acc.0 += 1;
            acc.1 += check_cycle(acc.0, acc.2);
            screen = draw_sprite_on_screen(&screen, acc.2, acc.0);
            acc.2 += val.trim().parse::<i32>().unwrap();
        }

        acc
    });

    (signal, screen)
}

pub fn part1() -> i32 {
    solve().0
}

pub fn part2() -> String {
    let screen = solve().1;
    let mut string = String::from("\n");

    for r in 0..screen.len() {
        for c in 0..screen[0].len() {
            string.push(screen[r][c]);
        }
        string.push('\n');
    }

    string
}
