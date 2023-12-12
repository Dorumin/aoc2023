use aoc2023::day2::Game;

const INPUT: &str = include_str!("../inputs/day2-cubes.txt");

fn main() {
    let total: i32 = INPUT.lines()
        .map(|line| Game::parse(line).expect("Games are valid lol"))
        .map(|game| game.get_minimum(|p| p.red) * game.get_minimum(|p| p.green) * game.get_minimum(|p| p.blue))
        .sum();

    dbg!(total);
}