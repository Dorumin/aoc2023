use aoc2023::day3::{SurroundScanner, Block};

const INPUT: &str = include_str!("../inputs/day3-gears.txt");

use itertools::Itertools;
use regex::Regex;

fn is_symbol(line: usize, index: usize) -> bool {
    match INPUT.lines().nth(line) {
        Some(s) => match s.get(index..(index + 1)) {
            Some(s) => s.starts_with(|c: char| c != '.' && !c.is_numeric()),
            None => false
        },
        None => false
    }
}

fn has_adjacent_symbol(line: usize, index: usize, len: usize) -> bool {
    if (index != 0 && is_symbol(line, index - 1)) || is_symbol(line, index + len) {
        return true
    }

    if (line != 0 && ((index != 0 && is_symbol(line - 1, index - 1)) || is_symbol(line - 1, index + len)))
    || (index != 0 && is_symbol(line + 1, index - 1)) || is_symbol(line + 1, index + len) {
        return true
    }

    for i in 0..len {
        if (line != 0 && is_symbol(line - 1, index + i)) || is_symbol(line + 1, index + i) {
            return true
        }
    }

    false
}

fn main() {
    // let total: i32 = INPUT.lines()
    //     .enumerate()
    //     .flat_map(|(linenumber, line)| regex.find_iter(line).map(move |cap| (linenumber, cap)))
    //     .filter(|(linenumber, capt)| has_adjacent_symbol(*linenumber, capt.start(), capt.len()))
    //     .inspect(|(ln, m)| println!("{}", m.as_str()))
    //     .map(|(ln, capt)| capt.as_str().parse::<i32>().unwrap())
    //     .sum();

    // dbg!(total);

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
        .inspect(|(block, surrounding_numbers)| {
            if surrounding_numbers.len() >= 2 {
                // dbg!(surrounding_numbers);
                eprintln!("x: {} y: {}", block.x, block.y);
                eprintln!("{:?}", scanner.surrounding(block));
            }
        })
        .map(|(_, surrounding_numbers)| surrounding_numbers)
        .filter(|surrounding_numbers| surrounding_numbers.len() >= 2)
        .map(|surrounding_numbers| surrounding_numbers.iter().map(|n| n.1 as i64).reduce(|total, number| total * number).unwrap())
        .sum();

    dbg!(total);
}