const INPUT: &str = include_str!("../inputs/day3-gears.txt");

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
    let regex = Regex::new(r"[0-9]+").unwrap();
    let total: i32 = INPUT.lines()
        .enumerate()
        .flat_map(|(linenumber, line)| regex.find_iter(line).map(move |cap| (linenumber, cap)))
        .filter(|(linenumber, capt)| has_adjacent_symbol(*linenumber, capt.start(), capt.len()))
        .inspect(|(ln, m)| println!("{}", m.as_str()))
        .map(|(ln, capt)| capt.as_str().parse::<i32>().unwrap())
        .sum();

    dbg!(total);
}