use std::fmt::{Display, Write};

pub struct Cosmos {
    rows: Vec<Vec<Pixel>>
}

impl Cosmos {
    pub fn parse(input: &str) -> Option<Cosmos> {
        let rows = input.lines()
            .map(|line|
                line.chars()
                    .map(|c| Pixel::from_char(c).unwrap())
                    .collect()
            )
            .collect();

        Some(Cosmos {
            rows
        })
    }

    pub fn expand_empty_lines(&mut self) {
        // Expand dong, I mean rows. Flat map is an easy way to expand individual items into multiple
        // I don't know how well it does at doing this in-place. Likely not at all
        // But it's better than a for loop which mutates the array

        // Sneaky ownership stealing
        let rows = std::mem::take(&mut self.rows);

        self.rows = rows.into_iter()
            .flat_map(|row| {
                let row_is_empty = row.iter().all(|p| p.is_empty());
                std::iter::repeat(row).take(if row_is_empty { 2 } else { 1 })
            })
            .collect();

        // Expand the columns, which is only mildly more annoying
        let row_length = self.rows.first().expect("Fuck you there is at least one row").len();
        for i in row_length..0 {

        }
    }
}

impl Display for Cosmos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for pixel in row {
                pixel.fmt(f)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
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

    fn is_empty(&self) -> bool {
        matches!(self, Pixel::Empty)
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