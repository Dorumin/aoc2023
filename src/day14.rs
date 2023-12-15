use std::fmt::{Display, Write};

#[derive(Debug)]
pub struct Platform {
    pub rows: Vec<Vec<Item>>
}

impl Platform {
    pub fn parse(input: &str) -> Option<Self> {
        let rows = input.lines()
            .map(|line|
                line.chars()
                    .map(|c| Item::from_char(c).unwrap())
                    .collect()
            )
            .collect();

        Some(Self {
            rows
        })
    }

    fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                for y in 0..self.rows.len() {
                    for x in 0..self.width() {
                        self.slide_tile(x, y, direction.clone());
                    }
                }
            },
            Direction::Down => todo!(),
            Direction::Left => todo!(),
            Direction::Right => todo!(),
        }
    }

    pub fn total_load(&self) -> usize {
        let mut total = 0;

        for (i, row) in self.rows.iter().enumerate() {
            let load_factor = self.rows.len() - i;

            for cell in row.iter() {
                if *cell == Item::RoundedRock {
                    total += load_factor;
                }
            }
        }

        total
    }

    fn slide_tile(&mut self, x: usize, y: usize, direction: Direction) {
        let mut x = x;
        let mut y = y;

        if self.rows[y][x] != Item::RoundedRock {
            return;
        }

        // assert_eq!(self.rows[y][x], Item::RoundedRock);

        loop {
            // These better be fucking valid
            let current_cell = (x, y);
            let (new_x, new_y) = match direction {
                Direction::Up => (Some(x), y.checked_sub(1)),
                Direction::Down => (Some(x), y.checked_add(1)),
                Direction::Left => (x.checked_sub(1), Some(y)),
                Direction::Right => (x.checked_add(1), Some(y)),
            };

            if let (Some(new_x), Some(new_y)) = (new_x, new_y) {
                x = new_x;
                y = new_y;
            } else {
                break;
            }

            let next_cell = self.rows.get_mut(y).and_then(|row| row.get_mut(x));

            match next_cell {
                Some(item @ Item::Empty) => {
                    *item = Item::RoundedRock;
                    self.rows[current_cell.1][current_cell.0] = Item::Empty;
                },
                _ => break
            }
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for item in row.iter() {
                f.write_char(item.to_char())?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Empty,
    RoundedRock,
    CubeRock
}

impl Item {
    fn from_char(c: char) -> Option<Self> {
        let item = match c {
            'O' => Item::RoundedRock,
            '#' => Item::CubeRock,
            '.' => Item::Empty,
            _ => return None
        };

        Some(item)
    }

    fn to_char(&self) -> char {
        match self {
            Item::Empty => '.',
            Item::RoundedRock => 'O',
            Item::CubeRock => '#',
        }
    }
}