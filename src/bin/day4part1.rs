use aoc2023::day4::Scratchcard;

const INPUT: &str = include_str!("../inputs/day4-scratchcards.txt");

fn main() {
    let points_total: i32 = INPUT.lines()
        .flat_map(Scratchcard::parse_line)
        .map(|card| card.points())
        .sum();

    dbg!(points_total);
}