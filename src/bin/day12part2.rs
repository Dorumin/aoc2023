use aoc2023::day12::Map;

const INPUT: &str = include_str!("../inputs/day12-springs.txt");

fn main() {
    let mut map = Map::parse(INPUT).unwrap();

    map.unfold();

    for row in map.rows.iter() {
        for rec in row.records.iter() {
            print!("{}", rec.to_char());
        }

        println!();
    }

    let rows_arrangements = map.rows.iter()
        .map(|row| row.possible_arrangements().len());

    let sum: usize = rows_arrangements.sum();

    dbg!(sum);
}