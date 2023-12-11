use std::fmt::{Display, Write};

const WIDE_DISPLAY: bool = true;

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    positions: Vec<Position>
}

impl Map {
    pub fn parse(input: &str) -> Option<Map> {
        let mut width = 0;
        let mut height = 0;
        let mut positions = Vec::new();

        for line in input.lines() {
            for (x, c) in line.char_indices() {
                positions.push(Position {
                    x,
                    y: height,
                    tile: Tile::from_char(c).unwrap()
                });
            }

            width = line.len();
            height += 1;
        }

        Some(Map {
            width,
            height,
            positions
        })
    }

    pub fn start(&self) -> &Position {
        self.positions.iter().find(|&p| p.tile == Tile::Start).unwrap()
    }

    pub fn adjacent(&self, position: &Position) -> Vec<&Position> {
        let mut offsets = vec![];

        if let Tile::Start | Tile::Vertical | Tile::NorthEast | Tile::NorthWest = position.tile {
            offsets.push((Some(position.x), position.y.checked_sub(1)));
        }

        if let Tile::Start | Tile::Vertical | Tile::SouthEast | Tile::SouthWest = position.tile {
            offsets.push((Some(position.x), position.y.checked_add(1)));
        }

        if let Tile::Start | Tile::Horizontal | Tile::SouthWest | Tile::NorthWest = position.tile {
            offsets.push((position.x.checked_sub(1), Some(position.y)));
        }

        if let Tile::Start | Tile::Horizontal | Tile::SouthEast | Tile::NorthEast = position.tile {
            offsets.push((position.x.checked_add(1), Some(position.y)));
        }

        offsets.into_iter()
            .filter_map(|p| Some((p.0?, p.1?)))
            .map(|(x, y)| {
                self.get(x, y)
            })
            .collect()
    }

    pub fn get(&self, x: usize, y: usize) -> &Position {
        self.positions.iter().find(|p| p.x == x && p.y == y).unwrap()
    }

    pub fn follow<'map>(&'map self, start: &'map Position) -> Option<Vec<&'map Position>> {
        let mut starts = self.adjacent(start);
        starts.retain(|position| self.adjacent(position).contains(&start));

        for search_start in starts.iter() {
            let mut current = *search_start;
            let mut path = vec![start, current];

            loop {
                let mut nexts = self.adjacent(current).into_iter()
                    .filter(|&n| !path.iter().rev().any(|&p| p == n));
                let next = nexts.next();

                // dbg!(current, next);
                assert_eq!(nexts.next(), None);

                if let Some(next) = next {
                    current = next;
                    path.push(current);

                    if starts.contains(&next) {
                        return Some(path);
                    }
                } else {
                    break;
                }
            }
        }

        None
    }

    pub fn display_path(&self, path: &[&Position]) -> String {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let pos = &self.positions[i];

                if path.contains(&pos) {
                    s.push_str(&format!("{}", pos.tile));
                } else {
                    s.push(' ');
                }

                if WIDE_DISPLAY {
                    s.push(' ');
                }
            }

            s.push('\n');
        }

        s
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let pos = &self.positions[i];

                pos.tile.fmt(f)?;

                if WIDE_DISPLAY {
                    f.write_char(' ')?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    tile: Tile
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        let tile = match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => return None
        };

        Some(tile)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Vertical => '|',
            Tile::Horizontal => '─',
            Tile::NorthEast => '└',
            Tile::NorthWest => '┘',
            Tile::SouthEast => '┌',
            Tile::SouthWest => '┐',
            Tile::Ground => '•', // ■ •
            Tile::Start => '★', // ⌂
        };

        f.write_char(c)
    }
}