use std::time::Instant;

use aoc2023::day14::{Platform, Direction};

const INPUT: &str = include_str!("../inputs/day14-platform.txt");

fn main() {
    let mut platform = Platform::parse(INPUT).unwrap();

    // println!("{}", platform);

    let start = Instant::now();
    platform.tilt(Direction::Up);
    eprintln!("{:?}", start.elapsed());

    // println!("{}", platform);

    dbg!(platform.total_load());
}