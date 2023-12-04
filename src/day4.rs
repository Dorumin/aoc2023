use once_cell::sync::Lazy;

use regex::Regex;

#[derive(Clone)]
pub struct Scratchcard {
    pub game_id: usize,
    pub winning_numbers: Vec<i32>,
    pub gotten_numbers: Vec<i32>
}

impl Scratchcard {
    pub fn parse_line(line: &str) -> Option<Scratchcard> {
        static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
        static CARD_LINE_REGEX: Lazy<Regex> = Lazy::new(||
            Regex::new(r"^Card\s*(?<id>\d+):\s*(?<winning>.*?)*\s*\|\s*(?<numbers>.*?)\s*$").unwrap()
        );

        let captures = CARD_LINE_REGEX.captures(line).unwrap();

        let game_id = captures.name("id").unwrap().as_str().parse().ok()?;
        let winning_numbers = WHITESPACE_REGEX.split(captures.name("winning").unwrap().as_str()).flat_map(|s| s.parse()).collect();
        let gotten_numbers = WHITESPACE_REGEX.split(captures.name("numbers").unwrap().as_str()).flat_map(|s| s.parse()).collect();

        Some(Scratchcard {
            game_id,
            winning_numbers,
            gotten_numbers
        })
    }

    pub fn gotten_winning_numbers(&self) -> usize {
        self.gotten_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count()
    }

    pub fn points(&self) -> i32 {
        let count = self.gotten_winning_numbers();

        if count == 0 {
            return 0;
        } else {
            return 1 << (count - 1);
        }
    }
}