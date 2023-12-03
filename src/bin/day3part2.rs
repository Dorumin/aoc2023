use aoc2023::day3::{SurroundScanner, Block};

const INPUT: &str = include_str!("../inputs/day3-gears.txt");

use itertools::Itertools;
use regex::Regex;

fn main() {
    let numbers_regex = Regex::new(r"[0-9]+").unwrap();
    let scanner = SurroundScanner::new(INPUT);

    let numbers: Vec<_> = scanner.lines.iter()
        .enumerate()
        .flat_map(|(line_index, line)| numbers_regex.find_iter(line).map(move |m| (line_index, m)))
        .map(|(line_index, m)| (line_index, m.as_str().parse::<i32>().unwrap(), Block {
            x: m.start(),
            y: line_index,
            width: m.as_str().len(),
            height: 1
        }))
        .collect();

    let total: i64 = scanner.lines.iter()
        .enumerate()
        .flat_map(|(line_index, line)| line.match_indices('*').map(move |(index, _)| (line_index, index)))
        .map(|(line_index, index)| Block {
            x: index,
            y: line_index,
            width: 1,
            height: 1
        })
        .map(|block| {
            let surrounding_numbers: Vec<_> =  scanner.surrounding(&block).into_iter()
                .filter(|c| c.2.is_numeric())
                .map(|c| numbers.iter().find(|n| n.2.contains(c.1, c.0)).unwrap())
                .unique()
                .collect();

            (block, surrounding_numbers)
        })
        .map(|(_, surrounding_numbers)| surrounding_numbers)
        .filter(|surrounding_numbers| surrounding_numbers.len() >= 2)
        .map(|surrounding_numbers| surrounding_numbers.iter().map(|n| n.1 as i64).reduce(|total, number| total * number).unwrap())
        .sum();

    dbg!(total);
}