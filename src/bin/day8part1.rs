use aoc2023::day8::Map;

const INPUT: &str = include_str!("../inputs/day8-dromedary.txt");

fn main() {
    let map = Map::parse(INPUT).unwrap();

    dbg!(map.walk().len());
}