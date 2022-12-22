use itertools::Itertools;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("../../input/day21.txt");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Node {
    Literal(i128),
    Ident(String),
    BinaryOp(char),
    Expression(Vec<Node>),
}

fn parse_input() -> FxHashMap<Node, Node> {
    INPUT.lines().fold(FxHashMap::default(), |mut acc, l| {
        let (key, value) = l.split_once(':').unwrap();

        let key = Node::Ident(key.to_string());
        let value = match value.split_whitespace().collect_vec().as_slice() {
            [v] if v.parse::<usize>().is_ok() => Node::Literal(v.parse().unwrap()),
            [lhs, op, rhs] => {
                let lhs = Node::Ident(lhs.to_string());
                let op = Node::BinaryOp(op.as_bytes()[0] as char);
                let rhs = Node::Ident(rhs.to_string());

                Node::Expression(vec![lhs, op, rhs])
            }
            _ => unreachable!(),
        };

        acc.insert(key, value);
        acc
    })
}

fn evaluate(exprs: &FxHashMap<Node, Node>, start: &Node, detect_humn: bool) -> Option<f64> {
    if let Node::Ident(name) = start {
        if name == "humn" && detect_humn {
            return None;
        }
    }

    match exprs.get(start) {
        Some(Node::Literal(v)) => Some(*v as f64),
        Some(Node::Expression(nodes)) => {
            let lhs = evaluate(exprs, &nodes[0], detect_humn);
            let rhs = evaluate(exprs, &nodes[2], detect_humn);

            if lhs.is_none() || rhs.is_none() {
                return None;
            }

            if let Node::BinaryOp(op) = nodes[1] {
                match op {
                    '+' => Some(lhs.unwrap() + rhs.unwrap()),
                    '-' => Some(lhs.unwrap() - rhs.unwrap()),
                    '*' => Some(lhs.unwrap() * rhs.unwrap()),
                    '/' => Some(lhs.unwrap() / rhs.unwrap()),
                    _ => unreachable!(),
                }
            } else {
                panic!("Unexpected binary operator: {:?}", nodes[1]);
            }
        }
        _ => unreachable!(),
    }
}

pub fn part1() -> f64 {
    let input = parse_input();
    evaluate(&input, &Node::Ident("root".to_string()), false).unwrap()
}

pub fn part2() -> f64 {
    let input = parse_input();

    let exprs = match input.get(&Node::Ident("root".to_string())) {
        Some(Node::Expression(v)) => v.as_slice(),
        _ => unreachable!(),
    };

    let left = evaluate(&input, &exprs[0], true);
    let right = evaluate(&input, &exprs[2], true);

    let (num, mut branch) = if left.is_some() {
        (left, &exprs[2])
    } else {
        (right, &exprs[0])
    };

    let mut humn = num.unwrap();
    while let Some(Node::Expression(v)) = input.get(branch) {
        let left = evaluate(&input, &v[0], true);
        let right = evaluate(&input, &v[2], true);

        let (has_humn, constant) = if left.is_some() {
            (&v[2], left.unwrap())
        } else {
            (&v[0], right.unwrap())
        };

        humn = if let Node::BinaryOp(op) = &v[1] {
            match op {
                '+' => humn - constant,
                '-' => {
                    if left.is_some() {
                        constant - humn
                    } else {
                        humn + constant
                    }
                }
                '*' => humn / constant,
                '/' => {
                    if left.is_some() {
                        constant / humn
                    } else {
                        humn * constant
                    }
                }
                _ => unreachable!(),
            }
        } else {
            panic!("Unexpected binary operator: {:?}", &v[1]);
        };

        if !v.contains(&Node::Ident("humn".to_string())) {
            branch = has_humn;
        } else {
            break;
        }
    }

    humn
}
