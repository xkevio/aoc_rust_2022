use itertools::Itertools;
use rustc_hash::{FxHashSet, FxHashMap};

const INPUT: &str = include_str!("../../input/day22.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TileType {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Tile {
    t_type: TileType,
    pos: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

macro_rules! move_dir {
    ($map:ident, $tile:ident, $x:expr, $y:expr, $dir:expr, $len:expr) => {
        for _ in 0..$len {
            if $map.contains(&Tile::new(TileType::Wall, $x, $y)) {
                return;
            }

            if $map.contains(&Tile::new(TileType::Open, $x, $y)) {
                $tile.pos = ($x, $y);
            } else {
                let smallest_pos = $tile.get_wrapped_tile($map, $dir);

                if let Some(t) = smallest_pos {
                    if t.t_type != TileType::Wall {
                        $tile.pos = t.pos;
                    } else {
                        return;
                    }
                }
            }
        }
    };
}

impl Tile {
    pub fn new(t_type: TileType, x: usize, y: usize) -> Self {
        Self {
            t_type,
            pos: (x, y),
        }
    }

    pub fn move_tile(&mut self, map: &FxHashSet<Tile>, current_dir: &Direction, len: usize) {
        match current_dir {
            Direction::Right => move_dir!(map, self, self.pos.0 + 1, self.pos.1, current_dir, len),
            Direction::Left => move_dir!(map, self, self.pos.0 - 1, self.pos.1, current_dir, len),
            Direction::Down => move_dir!(map, self, self.pos.0, self.pos.1 + 1, current_dir, len),
            Direction::Up => move_dir!(map, self, self.pos.0, self.pos.1 - 1, current_dir, len),
        }
    }

    pub fn move_tile_3d(&mut self, _regions: &FxHashMap<Tile, char>, _current_dir: &Direction, _len: usize) {
        todo!()
    }

    fn get_wrapped_tile(&self, map: &FxHashSet<Tile>, dir: &Direction) -> Option<Tile> {
        map.iter()
            .filter(|t| match dir {
                Direction::Left | Direction::Right => t.pos.1 == self.pos.1,
                Direction::Up | Direction::Down => t.pos.0 == self.pos.0,
            })
            .min_by(|a, b| match dir {
                Direction::Left | Direction::Up => b.pos.cmp(&a.pos),
                Direction::Right | Direction::Down => a.pos.cmp(&b.pos),
            })
            .copied()
    }

    fn get_3d_wrapped_tile(&self, _regions: &FxHashMap<Tile, char>, _current_dir: &Direction, _current_region: char) -> Option<Tile> {
        todo!()
    }
}

impl Direction {
    pub fn rotate(&self, clockwise: bool) -> Direction {
        match &self {
            Direction::Left => {
                if clockwise {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Right => {
                if clockwise {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Up => {
                if clockwise {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Down => {
                if clockwise {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
        }
    }
}

fn parse_input() -> (FxHashSet<Tile>, String) {
    let (map, instrs) = INPUT.split_once("\n\n").unwrap();

    let map = map
        .lines()
        .enumerate()
        .fold(FxHashSet::<Tile>::default(), |mut acc, (y, l)| {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '.' => acc.insert(Tile::new(TileType::Open, x + 1, y + 1)),
                    '#' => acc.insert(Tile::new(TileType::Wall, x + 1, y + 1)),
                    _ => continue,
                };
            }

            acc
        });

    (map, instrs.to_string())
}

fn apply_instructions(map: &FxHashSet<Tile>, instr: &str, start: &Tile, regions: Option<&FxHashMap<Tile, char>>) -> usize {
    let lengths = instr.split(char::is_alphabetic).collect_vec();
    let mut directions = instr.split(char::is_numeric).collect_vec();

    directions.insert(0, "_");
    directions.retain(|a| !a.is_empty());

    let mut current_dir = Direction::Right;
    let mut new_tile = *start;

    for (dir, l) in directions.iter().zip(lengths.iter()) {
        let len = l.parse::<usize>().unwrap();

        current_dir = match *dir {
            "R" => current_dir.rotate(true),
            "L" => current_dir.rotate(false),
            _ => current_dir,
        };

        if let Some(r) = regions {
            new_tile.move_tile_3d(r, &current_dir, len);
        } else {
            new_tile.move_tile(map, &current_dir, len);
        }
    }

    1000 * new_tile.pos.1 + 4 * new_tile.pos.0 + current_dir as usize
}

pub fn part1() -> usize {
    let (map, instrs) = parse_input();

    let start_pos = map
        .iter()
        .filter(|t| t.t_type == TileType::Open)
        .min_by(|x, y| x.pos.1.cmp(&y.pos.1).then_with(|| x.pos.0.cmp(&y.pos.0)));

    apply_instructions(&map, &instrs, start_pos.unwrap(), None)
}

pub fn part2() -> usize {
    // let (map, instrs) = parse_input();

    // let mut regions = FxHashMap::<Tile, char>::default();
    // for tile in &map {
    //     match tile.pos {
    //         (51..=100, 1..=50) => { regions.insert(*tile, 'A'); },
    //         (101.., 1..=50) => { regions.insert(*tile, 'B'); },
    //         (51..=100, 51..=100) => { regions.insert(*tile, 'C'); },
    //         (51..=100, 101..=150) => { regions.insert(*tile, 'D'); },
    //         (1..=50, 101..=150) => { regions.insert(*tile, 'E'); },
    //         (1..=50, 151..) => { regions.insert(*tile, 'E'); },
    //         _ => unreachable!("Didn't account for {:?}", tile.pos)
    //     }
    // }

    // let start_pos = map
    //     .iter()
    //     .filter(|t| t.t_type == TileType::Open)
    //     .min_by(|x, y| x.pos.1.cmp(&y.pos.1).then_with(|| x.pos.0.cmp(&y.pos.0)));

    // apply_instructions(&map, &instrs, start_pos.unwrap(), Some(&regions))    
    0
}
