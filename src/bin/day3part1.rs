use aoc2023::day3::{Block, SurroundScanner};
use regex::Regex;

const INPUT: &str = include_str!("../inputs/day3-gears.txt");

fn main() {
    let numbers_regex = Regex::new(r"[0-9]+").unwrap();
    let scanner = SurroundScanner::new(INPUT);

    let numbers = scanner.lines.iter()
        .enumerate()
        .flat_map(|(line_index, line)| numbers_regex.find_iter(line).map(move |m| (line_index, m)))
        .map(|(line_index, m)| (line_index, m.as_str().parse::<i32>().unwrap(), Block {
            x: m.start(),
            y: line_index,
            width: m.as_str().len(),
            height: 1
        }));

    let total: i32 = numbers
        .filter(|number| scanner.surrounding(&number.2).iter().any(|c| c.2 != '.' && !c.2.is_numeric()))
        .map(|(_, num, _)| num)
        .sum();

    dbg!(total);
}