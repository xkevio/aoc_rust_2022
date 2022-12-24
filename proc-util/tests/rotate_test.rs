use proc_util::RotateEnum;

#[derive(RotateEnum, Debug, PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[test]
fn next_test() {
    let start_dir = Dir::Right;
    assert_eq!(start_dir.next(), Dir::Down)
}

#[test]
fn prev_test() {
    let start_dir = Dir::Right;
    assert_eq!(start_dir.prev(), Dir::Up)
}
