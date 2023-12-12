use aoc2023::day11::Cosmos;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/day11-cosmos.txt");

fn main() {
    let mut cosmos = Cosmos::parse(INPUT).unwrap();

    // eprintln!("{cosmos}");

    cosmos.expand_empty_lines(999999);

    // eprintln!("{cosmos}");

    let total_distance: usize = cosmos.positions().iter()
        .filter(|tile| tile.pixel.is_galaxy())
        .tuple_combinations()
        .map(|(a, b)| a.distance(b))
        .sum();

    dbg!(total_distance);
}