use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("../../input/day12.txt");

type Element = ((usize, usize), char);

fn parse_input() -> Vec<Vec<Element>> {
    INPUT
        .lines()
        .enumerate()
        .map(|(row, c)| {
            c.chars()
                .enumerate()
                .map(|(col, v)| ((row, col), v))
                .collect_vec()
        })
        .collect()
}

fn get_possible_neighbors(current_element: &Element, grid: &[Vec<Element>]) -> Vec<Element> {
    let mut neighbors = Vec::<Element>::new();
    let (row, col) = current_element.0;
    let value = current_element.1;

    for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (row, col) = (row as i32 + offset.0, col as i32 + offset.1);
        if let Some(v) = grid.get(row as usize).and_then(|r| r.get(col as usize)) {
            if v.1 as u8 <= value as u8 + 1 {
                neighbors.push(*v);
            }
        }
    }

    neighbors
}

fn bfs(start_element: &Element, end_element: &Element, grid: &[Vec<Element>]) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::<Element, Element>::new();
    let mut explored = HashSet::<Element>::new();

    explored.insert(*start_element);
    queue.push_back(*start_element);

    while !queue.is_empty() {
        let v = queue.pop_front();

        if let Some(vv) = v {
            if vv.0 == end_element.0 {
                let mut cur = vv;
                let mut counter = 0;

                while let Some(e) = distances.get(&cur) {
                    counter += 1;
                    cur = *e;
                }

                return Some(counter);
            }

            for n in get_possible_neighbors(&vv, grid) {
                if !explored.contains(&n) {
                    explored.insert(n);
                    distances.insert(n, vv);
                    queue.push_back(n);
                }
            }
        }
    }

    None
}

#[rustfmt::skip]
fn solve(only_start: bool) -> usize {
    let mut grid = parse_input();
    let mut start_element = *grid.iter().find_map(|v| v.iter().find(|e| e.1 == 'S')).unwrap();
    let mut end_element = *grid.iter().find_map(|v| v.iter().find(|e| e.1 == 'E')).unwrap();

    start_element.1 = 'a';
    end_element.1 = 'z';

    grid[start_element.0.0][start_element.0.1] = start_element;
    grid[end_element.0.0][end_element.0.1] = end_element;

    if only_start {
        bfs(&start_element, &end_element, &grid).unwrap_or_default()
    } else {
        grid.iter()
            .flat_map(|e| {
                e.iter()
                    .filter(|e| e.1 == 'a')
                    .map(|start| bfs(start, &end_element, &grid).unwrap_or(usize::MAX))
            })
            .min()
            .unwrap()
    }
}

pub fn part1() -> usize {
    solve(true)
}

pub fn part2() -> usize {
    solve(false)
}
