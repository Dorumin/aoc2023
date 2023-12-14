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

    let mut args = std::env::args().skip(1);
    let skip_count = args.next().unwrap().parse().unwrap();
    let end_position: usize = args.next().unwrap().parse().unwrap();
    let rows_arrangements = map.rows.iter()
        .skip(skip_count)
        .take(end_position - skip_count)
        .map(|row| row.possible_arrangements_count());

    let sum: u64 = rows_arrangements.sum();

    dbg!(sum);
}