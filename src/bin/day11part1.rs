use aoc2023::day11::Cosmos;

const INPUT: &str = include_str!("../inputs/day11-cosmos.txt");

fn main() {
    let mut cosmos = Cosmos::parse(INPUT).unwrap();

    eprintln!("{cosmos}");

    cosmos.expand_empty_lines();

    eprintln!("{cosmos}");
}