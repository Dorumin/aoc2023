use aoc2023::day16::{Layout, Beam, Direction};

const INPUT: &str = include_str!("../inputs/day16-layout.txt");

fn main() {
    let layout = Layout::parse(INPUT).unwrap();

    let mut start_beam = Beam::new(Direction::Right, 0, 0);

    let visited_positions = start_beam.drive(&layout);

    dbg!(visited_positions.len());
}