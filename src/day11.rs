use std::fmt::{Display, Write};

pub struct Cosmos {
    rows: Vec<Row>
}

pub struct Row {
    x: usize,
    cells: Vec<Cell>
}

impl Row {
    fn is_empty(&self) -> bool {
        self.cells.iter().all(|c| c.pixel.is_empty())
    }
}

pub struct Cell {
    y: usize,
    pixel: Pixel
}

impl Cosmos {
    pub fn parse(input: &str) -> Option<Cosmos> {
        let rows = input.lines()
            .enumerate()
            .map(|(x, line)|
                Row {
                    x,
                    cells: line.chars()
                        .enumerate()
                        .map(|(y, c)| Cell {
                            y,
                            pixel: Pixel::from_char(c).unwrap()
                        })
                        .collect()
                }
            )
            .collect();

        Some(Cosmos {
            rows
        })
    }

    pub fn expand_empty_lines(&mut self, multiplier: usize) {
        // Expand dong, I mean rows. Flat map is an easy way to expand individual items into multiple
        // I don't know how well it does at doing this in-place. Likely not at all
        // But it's better than a for loop which mutates the array

        let mut accumulated_row_increment = 0;
        for row in self.rows.iter_mut() {
            if row.is_empty() {
                accumulated_row_increment += multiplier;
            }

            row.x += accumulated_row_increment;
        }

        let mut accumulated_column_increment = 0;
        let row_length = self.rows.first().expect("Fuck you there is at least one row").cells.len();
        for i in 0..row_length {
            if self.rows.iter().all(|row| row.cells[i].pixel.is_empty()) {
                accumulated_column_increment += multiplier;
            }

            for row in self.rows.iter_mut() {
                row.cells[i].y += accumulated_column_increment;
            }
        }
    }

    pub fn positions(&self) -> Vec<Tile> {
        self.rows.iter()
            .flat_map(|row|
                row.cells.iter().map(move |cell| Tile {
                    x: row.x,
                    y: cell.y,
                    pixel: cell.pixel.clone()
                })
            )
            .collect()
    }
}

impl Display for Cosmos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for cell in row.cells.iter() {
                cell.pixel.fmt(f)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub pixel: Pixel
}

impl Tile {
    pub fn new(x: usize, y: usize, pixel: Pixel) -> Self{
        Self {
            x,
            y,
            pixel
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
pub enum Pixel {
    Empty,
    Galaxy
}

impl Pixel {
    fn from_char(c: char) -> Option<Pixel> {
        let pixel = match c {
            '.' => Pixel::Empty,
            '#' => Pixel::Galaxy,
            _ => return None
        };

        Some(pixel)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Pixel::Empty)
    }

    pub fn is_galaxy(&self) -> bool {
        matches!(self, Pixel::Galaxy)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pixel::Empty => '.',
            Pixel::Galaxy => '#',
        };

        f.write_char(c)
    }
}

// struct __<T>(T);

// impl<T>__<T> {

// }