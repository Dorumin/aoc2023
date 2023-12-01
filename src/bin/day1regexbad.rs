use std::sync::OnceLock;

use regex::Regex;

const INPUT: &str = include_str!("../inputs/day1-trebuchet.txt");

#[derive(Eq, PartialEq, Debug)]
struct Calibration {
    line: &'static str,
    number: i32
}

static REGEX: OnceLock<Regex> = OnceLock::new();

impl Calibration {
    fn name_to_num(name: &str) -> &str {
        match name {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => name
        }
    }

    fn from_str(line: &'static str) -> Option<Self> {
        let regex = REGEX.get_or_init(|| Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap());

        let mut matches = regex.find_iter(line);
        let first = Self::name_to_num(matches.next()?.as_str());
        let last = Self::name_to_num(matches.last().map(|m| m.as_str()).unwrap_or_else(|| {
            eprintln!("no last: {line} = {first}{first}");
            first
        }));

        let n = format!("{first}{last}").parse().ok()?;

        Some(Calibration { line, number: n })
    }
}

fn main() {
    let total = INPUT.lines()
        .map(|line| Calibration::from_str(line))
        // .inspect(|c| eprintln!("{} = {}", c.as_ref().unwrap().line, c.as_ref().unwrap().number))
        .flat_map(|opt| opt)
        .fold(0, |n, c| n + c.number);

    dbg!(total);
}

#[cfg(test)]
mod tests {
    use crate::Calibration;

    #[test]
    fn simple() {
        let c = Calibration::from_str("1asd2");

        assert!(c.is_some());
        assert_eq!(c.unwrap().number, 12);
    }

    #[test]
    fn repeat() {
        let c = Calibration::from_str("nine");

        assert!(c.is_some());
        assert_eq!(c.unwrap().number, 99);
    }
}
