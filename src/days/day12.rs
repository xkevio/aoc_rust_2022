use std::collections::{HashMap, BinaryHeap};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day12.txt");

type Element = ((usize, usize), char);
type AdjacencyList = HashMap<Element, Vec<Element>>;

fn get_possible_neighbors(current_element: Element, grid: &[Vec<Element>]) -> Vec<Element> {
    let mut neighbors = Vec::<Element>::new();
    let (row, col) = current_element.0;
    let value = current_element.1;

    if let Some(up) = grid.get(row - 1) {
        if let Some(upp) = up.get(col) {
            if upp.1 as u8 == value as u8 + 1 {
                neighbors.push(*upp);
            }
        }
    }

    if let Some(up) = grid.get(row + 1) {
        if let Some(upp) = up.get(col) {
            if upp.1 as u8 == value as u8 + 1 {
                neighbors.push(*upp);
            }
        }
    }

    if let Some(up) = grid.get(row) {
        if let Some(upp) = up.get(col - 1) {
            if upp.1 as u8 == value as u8 + 1 {
                neighbors.push(*upp);
            }
        }
    }

    if let Some(up) = grid.get(row) {
        if let Some(upp) = up.get(col + 1) {
            if upp.1 as u8 == value as u8 + 1 {
                neighbors.push(*upp);
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
            let neighbors = get_possible_neighbors(*el, &grid);
            adjacency_list.insert(*el, neighbors);
        }
    }

    adjacency_list
}

fn bfs(start_element: Element, goal: Element, adjacency_list: &AdjacencyList) -> Option<usize> {
    let mut queue = BinaryHeap::<Element>::new();
    let mut distance: Option<usize> = None;

    

    distance
}

pub fn part1() -> usize {
    0
}

pub fn part2() -> usize {
    0
}
