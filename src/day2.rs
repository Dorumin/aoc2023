#[derive(Debug, Eq, PartialEq)]
pub struct Filter {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    pub id: i32,
    pub pulls: Vec<Pull>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pull {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

impl Filter {
    pub fn is_game_valid(&self, game: &Game) -> bool {
        return game.pulls.iter().all(|pull| self.is_pull_valid(pull));
    }

    pub fn is_pull_valid(&self, pull: &Pull) -> bool {
        pull.red <= self.red && pull.green <= self.green && pull.blue <= self.blue
    }
}

impl Game {
    pub fn get_minimum(&self, map: impl Fn(&Pull) -> i32) -> i32 {
        return self.pulls.iter().map(map).max().unwrap();
    }

    pub fn parse(line: &str) -> Option<Self> {
        let mut pulls = Vec::new();
        let mut line = line.strip_prefix("Game ")?;
        let id;
        (id, line) = get_number_prefix(line)?;
        line = line.strip_prefix(": ")?;

        while !line.is_empty() {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            for _ in 0..3 {
                if line.starts_with("; ") || line.is_empty() {
                    break;
                }

                let n;
                (n, line) = get_number_prefix(line)?;
                line = line.strip_prefix(' ')?;

                if let Some(rest) = line.strip_prefix("red") {
                    red = n;
                    line = rest;
                } else if let Some(rest) = line.strip_prefix("green") {
                    green = n;
                    line = rest;
                } else if let Some(rest) = line.strip_prefix("blue") {
                    blue = n;
                    line = rest;
                }

                if let Some(rest) = line.strip_prefix(", ") {
                    line = rest;
                }
            }

            pulls.push(Pull {
                red,
                green,
                blue
            });

            if let Some(rest) = line.strip_prefix("; ") {
                line = rest;
            } else {
                assert!(line.is_empty());
                break;
            }
        }

        Some(Game {
            id,
            pulls
        })
    }
}

pub fn get_number_prefix(s: &str) -> Option<(i32, &str)> {
    let rest = s.trim_start_matches(|c: char| c.is_numeric());

    if rest.len() == s.len() {
        None
    } else {
        let removed = s[..(s.len() - rest.len())].parse().ok()?;

        Some((removed, rest))
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn aoc_samples() {
        Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
        Game::parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap();
        Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap();
        Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
    }

    #[test]
    fn missing() {
        Game::parse("Game 1: 3 blue, 4 red, 1 green").unwrap();
        Game::parse("Game 1: 3 blue, 4 red").unwrap();
        Game::parse("Game 1: 3 blue").unwrap();
        Game::parse("Game 1: 4 red").unwrap();
        Game::parse("Game 1: 1 blue").unwrap();
    }

    #[test]
    fn repeats() {
        // This probably shouldn't work, but we don't care
        Game::parse("Game 1: 1 blue, 2 blue, 3 blue").unwrap();
    }

    #[test]
    #[should_panic]
    fn no_more_than_three() {
        Game::parse("Game 1: 1 blue, 2 blue, 3 blue, 4 blue, 5 blue").unwrap();
    }
}