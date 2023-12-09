use aoc2023::day9::Oasis;

const INPUT: &str = include_str!("../inputs/day9-oasis.txt");

fn main() {
    let oasis = Oasis::parse(INPUT).unwrap();

    let total: i32 = oasis.histories().iter().map(|hist| hist.predict_next_value().1).sum();

    dbg!(total);
}