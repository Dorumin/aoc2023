use aoc2023::day16::{Layout, Beam, Direction};

const INPUT: &str = include_str!("../inputs/day16-layout.txt");

fn main() {
    let layout = Layout::parse(INPUT).unwrap();

    let mut max_result = 0;

    for (x, y) in (0..layout.width()).zip(std::iter::repeat(0)) {
        max_result = max_result.max(Beam::new(Direction::Down, x, y).drive(&layout).len());
    }

    for (x, y) in (0..layout.width()).zip(std::iter::repeat(layout.height() - 1)) {
        max_result = max_result.max(Beam::new(Direction::Down, x, y).drive(&layout).len());
    }

    for (y, x) in (0..layout.height()).zip(std::iter::repeat(0)) {
        max_result = max_result.max(Beam::new(Direction::Down, x, y).drive(&layout).len());
    }

    for (y, x) in (0..layout.height()).zip(std::iter::repeat(layout.width() - 1)) {
        max_result = max_result.max(Beam::new(Direction::Down, x, y).drive(&layout).len());
    }

    dbg!(max_result);
}