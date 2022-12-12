use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("../../input/day12.txt");

type Element = ((usize, usize), char);
type AdjacencyList = HashMap<Element, Vec<Element>>;

fn get_possible_neighbors(current_element: Element, grid: &[Vec<Element>]) -> Vec<Element> {
    let mut neighbors = Vec::<Element>::new();
    let (row, col) = current_element.0;

    let transform_value = |c: char| {
        if c == 'S' {
            'a'
        } else if c == 'E' {
            'z'
        } else {
            c
        }
    };

    let value = transform_value(current_element.1);
    for offset in [-1, 1] {
        if let Some(up) = grid.get((row as i32 + offset) as usize) {
            let v_upp = transform_value(up[col].1);
            if v_upp as u8 <= value as u8 + 1 {
                neighbors.push(up[col]);
            }
        }
    }

    for offset in [-1, 1] {
        if let Some(up) = grid[row].get((col as i32 + offset) as usize) {
            let v_upp = transform_value(up.1);
            if v_upp as u8 <= value as u8 + 1 {
                neighbors.push(*up);
            }
        }
    }

    neighbors
}

fn build_adjacency_list() -> AdjacencyList {
    let mut adjacency_list = AdjacencyList::new();

    let grid = INPUT
        .lines()
        .enumerate()
        .map(|(row, c)| {
            c.chars()
                .enumerate()
                .map(|(col, v)| ((row, col), v))
                .collect_vec()
        })
        .collect_vec();

    for l in &grid {
        for el in l {
            adjacency_list.insert(*el, get_possible_neighbors(*el, &grid));
        }
    }

    adjacency_list
}

fn bfs(start_element: &Element, adjacency_list: &AdjacencyList) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::<Element, Element>::new();
    let mut explored = HashSet::<Element>::new();

    explored.insert(*start_element);
    queue.push_back(start_element);

    while !queue.is_empty() {
        let v = queue.pop_front();

        if let Some(vv) = v {
            if vv.1 == 'E' {
                let mut cur = vv;
                let mut counter = 0;

                while let Some(e) = distances.get(cur) {
                    counter += 1;
                    cur = e;
                }

                return Some(counter);
            }

            for n in adjacency_list.get(vv).unwrap() {
                if !explored.contains(n) {
                    explored.insert(*n);
                    distances.insert(*n, *vv);
                    queue.push_back(n);
                }
            }
        }
    }

    None
}

pub fn part1() -> usize {
    bfs(&((20, 0), 'S'), &build_adjacency_list()).unwrap_or_default()
}

pub fn part2() -> usize {
    let adjacency_list = build_adjacency_list();

    adjacency_list
        .keys()
        .filter(|k| k.1 == 'a')
        .map(|start| bfs(start, &adjacency_list).unwrap_or(usize::MAX))
        .min()
        .unwrap()
}
