const INPUT: &str = include_str!("../inputs/day1-trebuchet.txt");

#[derive(Eq, PartialEq, Debug)]
struct Calibration {
    line: &'static str,
    number: i32
}

const SEARCH: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

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

    fn find_in<'a>(haystack: &'a str, matches: &[&str]) -> &'a str {
        let first = matches.iter()
            .enumerate()
            .map(|(i, find)| (i, haystack.find(find)))
            .flat_map(|(i, matched)| matched.map(|x| (i, x)))
            .reduce(|smallest, current| if smallest.1 < current.1 { smallest } else { current })
            .unwrap();

        &haystack[first.1..(matches[first.0].len() + first.1)]
    }

    fn rfind_in<'a>(haystack: &'a str, matches: &[&str]) -> &'a str {
        let last = matches.iter()
            .enumerate()
            .map(|(i, find)| (i, haystack.rfind(find)))
            .flat_map(|(i, matched)| matched.map(|x| (i, x)))
            .reduce(|largest, current| if largest.1 > current.1 { largest } else { current })
            .unwrap();

        &haystack[last.1..(matches[last.0].len() + last.1)]
    }

    fn from_str(line: &'static str) -> Option<Self> {
        let first = Self::name_to_num(Self::find_in(line, &SEARCH));
        let last = Self::name_to_num(Self::rfind_in(line, &SEARCH));
        let n = format!("{first}{last}").parse().ok()?;

        Some(Calibration { line, number: n })
    }
}

fn main() {
    let total = INPUT.lines()
        .map(Calibration::from_str)
        .inspect(|c| eprintln!("{} = {}", c.as_ref().unwrap().line, c.as_ref().unwrap().number))
        .flatten()
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

    #[test]
    fn thanks_pux() {
        let c = Calibration::from_str("twone");

        assert!(c.is_some());
        assert_eq!(c.unwrap().number, 21);
    }
}
