use aoc2023::day15::Sequence;

const INPUT: &str = include_str!("../inputs/day15-steps.txt");

fn main() {
    let seq = Sequence::parse(INPUT).unwrap();

    let result: u64 = seq.steps.iter().map(|s| s.hash() as u64).sum();

    dbg!(result);
}