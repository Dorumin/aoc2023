use aoc2023::day5::{SeedsMap};

const INPUT: &str = include_str!("../inputs/day5-seeds.txt");

fn main() {
    let map = SeedsMap::parse(INPUT).unwrap();

    dbg!(map.transform_seeds().iter().map(|seed| seed.1).min());
}