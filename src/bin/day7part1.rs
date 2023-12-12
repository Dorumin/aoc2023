use aoc2023::day7::{Game, Ruleset};

const INPUT: &str = include_str!("../inputs/day7-cards.txt");

fn main() {
    let mut game = Game::parse(INPUT, Ruleset::Simple);

    game.plays.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let result = game.plays.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (index, play)| sum + play.bid * (index as u64 + 1));

    dbg!(result);
}
