use itertools::Itertools;
use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("../../input/day18.txt");

type Cube = (usize, usize, usize);

fn parse_input() -> Vec<Cube> {
    INPUT
        .lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .filter_map(|l| l.parse::<usize>().ok())
                .collect_tuple()
                .unwrap();
            (x + 1, y + 1, z + 1)
        })
        .collect()
}

fn get_neighbors(cube: &Cube, cubes: Option<&[Cube]>) -> Vec<Cube> {
    let mut neighbors = Vec::new();

    for offset in [-1, 1] {
        let x = ((cube.0 as i32 + offset) as usize, cube.1, cube.2);
        let y = (cube.0, (cube.1 as i32 + offset) as usize, cube.2);
        let z = (cube.0, cube.1, (cube.2 as i32 + offset) as usize);

        for el in [x, y, z] {
            if let Some(c) = cubes {
                if c.contains(&el) {
                    neighbors.push(el);
                }
            } else {
                neighbors.push(el);
            }
        }
    }

    neighbors
}

fn flood_fill(start: &Cube, cubes: &[Cube]) -> FxHashSet<Cube> {
    let mut air = FxHashSet::<Cube>::default();
    let mut neighbors = vec![*start];

    while let Some(n) = neighbors.pop() {
        if !air.contains(&n)
            && !cubes.contains(&n)
            && ((0..32).contains(&n.0) && (0..32).contains(&n.1) && (0..32).contains(&n.2))
        {
            air.insert(n);
            neighbors.append(&mut get_neighbors(&n, None));
        }
    }

    air
}

pub fn part1() -> usize {
    let cubes = parse_input();

    let diff: usize = cubes
        .iter()
        .map(|c| get_neighbors(c, Some(&cubes)).len())
        .sum();

    let all_sides = cubes.len() * 6;
    all_sides - diff
}

pub fn part2() -> usize {
    let cubes = parse_input();
    let air = Vec::from_iter(flood_fill(&(0, 0, 0), &cubes));

    cubes
        .iter()
        .map(|c| get_neighbors(c, Some(&air)).len())
        .sum::<usize>()
}
