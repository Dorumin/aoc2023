use aoc2023::day6::Races;

const INPUT: &str = include_str!("../inputs/day6-races.txt");

fn main() {
    let race = Races::parse(INPUT).unwrap().one_race();

    dbg!(race.possible_winning_scores().count());
}