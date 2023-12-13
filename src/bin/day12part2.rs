use aoc2023::day12::Map;

const INPUT: &str = include_str!("../inputs/day12-springs.txt");

fn main() {
    let mut map = Map::parse(INPUT).unwrap();

    map.unfold();

    // for row in map.rows.iter() {
    //     for rec in row.records.iter() {
    //         print!("{}", rec.to_char());
    //     }

    //     println!();
    // }

    let rows_arrangements = map.rows.iter()
        .skip(std::env::args().nth(1).unwrap().parse().unwrap())
        .map(|row| row.possible_arrangements_count());

    let sum: u64 = rows_arrangements.sum();

    dbg!(sum);
}