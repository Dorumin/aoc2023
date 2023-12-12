use aoc2023::day12::Map;

const INPUT: &str = include_str!("../inputs/day12-springs.txt");

fn main() {
    let map = Map::parse(INPUT).unwrap();

    let rows_arrangements = map.rows.iter()
        .map(|row| row.possible_arrangements_starts());

    let sum: usize = rows_arrangements.map(|arrangements| arrangements.len()).sum();

    dbg!(sum);
}