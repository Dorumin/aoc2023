use aoc2023::day6::Races;

const INPUT: &str = include_str!("../inputs/day6-races.txt");

fn main() {
    let races = Races::parse(INPUT).unwrap();

    dbg!(races.races().iter().map(|race| race.possible_winning_scores().count()).reduce(|total, count| total * count));
}