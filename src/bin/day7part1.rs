use aoc2023::day7::{Game, Play, Card};

const INPUT: &str = include_str!("../inputs/day7-cards.txt");

fn main() {
    let mut game = Game::parse(INPUT);

    let play1 = Play::parse_line("KK677 1").unwrap();
    let play2 = Play::parse_line("KTJJT 1").unwrap();

    game.plays.sort();

    let result = game.plays.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (index, play)| sum + play.bid * (index as u64 + 1));

    dbg!(result);
}
