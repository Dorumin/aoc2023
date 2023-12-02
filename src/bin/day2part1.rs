const INPUT: &str = include_str!("../inputs/day2-cubes.txt");

struct Filter {
    red: i32,
    green: i32,
    blue: i32
}

struct Game {
    id: i32,
    pulls: Vec<Pull>
}

struct Pull {
    red: i32,
    green: i32,
    blue: i32
}

impl Filter {
    fn is_game_valid(&self, game: &Game) -> bool {
        return game.pulls.iter().all(|pull| pull.red <= self.red && pull.green <= self.green && pull.blue <= self.blue);
    }
}

fn get_number_prefix<'a>(s: &'a str,) -> Option<(i32, &'a str)> {
    let rest = s.trim_start_matches(|c: char| c.is_numeric());

    if rest.len() == s.len() {
        None
    } else {
        let removed = s[..(s.len() - rest.len())].parse().ok()?;

        return Some((removed, rest));
    }
}

impl Game {
    fn from_str(line: &str) -> Option<Self> {
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
                line = line.strip_prefix(" ")?;

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

            // dbg!((red, green, blue));

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

fn main() {
    let filter = Filter {
        red: 12,
        green: 13,
        blue: 14
    };

    let total: i32 = INPUT.lines()
        .flat_map(|line| Game::from_str(line))
        .filter(|game| filter.is_game_valid(game))
        .map(|game| game.id)
        .sum();

    dbg!(total);
}