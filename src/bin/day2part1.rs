use aoc2023::day2::{Game, Filter};

const INPUT: &str = include_str!("../inputs/day2-cubes.txt");

fn main() {
    let filter = Filter {
        red: 12,
        green: 13,
        blue: 14
    };

    let total: i32 = INPUT.lines()
        .map(|line| Game::from_str(line).expect("Games are valid lol"))
        .filter(|game| filter.is_game_valid(game))
        .map(|game| game.id)
        .sum();

    dbg!(total);
}