pub struct Filter {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

pub struct Game {
    pub id: i32,
    pub pulls: Vec<Pull>
}

pub struct Pull {
    pub red: i32,
    pub green: i32,
    pub blue: i32
}

impl Game {
    pub fn get_minimum(&self, map: impl Fn(&Pull) -> i32) -> i32 {
        return self.pulls.iter().map(map).max().unwrap();
    }
}

impl Filter {
    pub fn is_game_valid(&self, game: &Game) -> bool {
        return game.pulls.iter().all(|pull| pull.red <= self.red && pull.green <= self.green && pull.blue <= self.blue);
    }
}

pub fn get_number_prefix<'a>(s: &'a str,) -> Option<(i32, &'a str)> {
    let rest = s.trim_start_matches(|c: char| c.is_numeric());

    if rest.len() == s.len() {
        None
    } else {
        let removed = s[..(s.len() - rest.len())].parse().ok()?;

        return Some((removed, rest));
    }
}

impl Game {
    pub fn from_str(line: &str) -> Option<Self> {
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
