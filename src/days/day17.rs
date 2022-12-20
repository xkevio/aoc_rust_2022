use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("../../input/day17.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Minus,
    Plus,
    ReversedL,
    Line,
    Square,
}

#[derive(Clone, Debug)]
struct Piece {
    shape: Shape,
    positions: Vec<(usize, usize)>,
}

impl Piece {
    pub fn new(shape: Shape, positions: &[(usize, usize)]) -> Self {
        Self {
            shape,
            positions: positions.to_vec(),
        }
    }
}

fn simulate_falling_pieces(stopped_rocks: usize) -> usize {
    let mut current_floor = 0;
    let mut rocks = 0;

    let mut has_fallen = FxHashSet::<usize>::default();

    let pieces = [
        Piece::new(Shape::Minus, &[(2, 3), (3, 3), (4, 3), (5, 3)]),
        Piece::new(Shape::Plus, &[(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)]),
        Piece::new(Shape::ReversedL, &[(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)]),
        Piece::new(Shape::Line, &[(2, 3), (2, 4), (2, 5), (2, 6)]),
        Piece::new(Shape::Square, &[(2, 3), (3, 3), (2, 4), (3, 4)]),
    ];

    while rocks < stopped_rocks {
        let mut piece_iter = pieces.iter().cycle();
        let mut piece = piece_iter.next().unwrap().clone();

        'p: for push in INPUT.chars().cycle() {
            // left, right push
            piece = match push {
                '>' => {
                    let mut new_positions = Vec::from(piece.positions.clone());
                    let max_width = piece.positions.iter().max_by_key(|a| a.0).unwrap();

                    if new_positions
                        .iter()
                        .all(|(x, y)| !has_fallen.get(&(y * 7 + (x + 1))).is_some())
                    {
                        for i in 0..piece.positions.len() {
                            let (x, y) = piece.positions[i];

                            if max_width.0 < 6 {
                                // println!("going right");
                                new_positions[i] = (x + 1, y);
                            }
                        }
                    }

                    Piece::new(piece.shape, &new_positions)
                }
                '<' => {
                    let mut new_positions = Vec::from(piece.positions.clone());
                    let min_width = piece.positions.iter().min_by_key(|a| a.0).unwrap();

                    if new_positions
                        .iter()
                        .all(|(x, y)| !has_fallen.get(&(y * 7 + (x - 1))).is_some())
                    {
                        for i in 0..piece.positions.len() {
                            let (x, y) = piece.positions[i];

                            if min_width.0 > 0 {
                                // println!("going left");
                                new_positions[i] = (x - 1, y);
                            }
                        }
                    }

                    Piece::new(piece.shape, &new_positions)
                }
                _ => unreachable!(),
            };

            // down push
            let mut new_positions = Vec::from(piece.positions.clone());
            let min_height = piece.positions.iter().min_by_key(|a| a.1).unwrap();
            let mut done = false;

            if piece
                .positions
                .iter()
                .all(|(x, y)| !has_fallen.get(&((y - 1) * 7 + x)).is_some())
            {
                for i in 0..piece.positions.len() {
                    let (x, y) = piece.positions[i];

                    if min_height.1 > 0
                        || (!has_fallen.is_empty() && !has_fallen.get(&((y - 1) * 7 + x)).is_some())
                    {
                        // println!("going down");
                        new_positions[i] = (x, y - 1);
                    } else {
                        done = true;
                        break;
                    }
                }

                piece = Piece::new(piece.shape, &new_positions);
            } else {
                done = true;
            }

            if done {
                // println!("done going down");
                current_floor =
                    current_floor.max(piece.positions.iter().max_by_key(|a| a.1).unwrap().1 + 1);
                // println!("{current_floor}");
                // println!("{:?}", &piece);

                piece.positions.iter().for_each(|p| {
                    let index = p.1 * 7 + p.0;
                    has_fallen.insert(index);
                });

                rocks += 1;
                // println!("{rocks}");

                piece = piece_iter.next().unwrap().clone();
                piece.positions = piece
                    .positions
                    .iter()
                    .map(|a| (a.0, a.1 + current_floor))
                    .collect();

                if rocks == stopped_rocks {
                    break 'p;
                }

                // done = false;
            }

            // println!("{current_floor}");
            // println!("{:?}", &piece);
        }
    }

    current_floor
}

pub fn part1() -> usize {
    let a = simulate_falling_pieces(2022);
    a
}

pub fn part2() -> usize {
    // let a = simulate_falling_pieces(1_000_000_000_000);
    // a
    0
}
