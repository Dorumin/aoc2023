use std::collections::HashSet;

pub use crate::common::Direction;

#[derive(Debug)]
pub struct Layout {
    pub rows: Vec<Vec<Tile>>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Beam {
    direction: Direction,
    x: usize,
    y: usize
}

impl Beam {
    pub fn new(direction: Direction, x: usize, y: usize) -> Self {
        Beam {
            direction,
            x,
            y,
        }
    }

    pub fn with_direction(&self, direction: Direction) -> Self {
        Self {
            direction,
            x: self.x,
            y: self.y
        }
    }

    pub fn drive(&mut self, layout: &Layout) -> HashSet<(usize, usize)> {
        let mut points = HashSet::new();
        let mut explored_beams = HashSet::new();

        self.drive_with_points(layout,  &mut explored_beams);

        for beam in explored_beams.into_iter() {
            points.insert((beam.x, beam.y));
        }

        points
    }

    pub fn drive_with_points(
        &mut self,
        layout: &Layout,
        explored_beams: &mut HashSet<Beam>
    ) {
        loop {
            let inserted = explored_beams.insert(self.clone());

            // If the coordinate and direction existed in the hash fart, break
            if !inserted {
                break;
            }

            // eprintln!("exploring by {:?} at {} {}", self.direction, self.x, self.y);

            match layout.get_tile_at_beam(self) {
                // In empty tiles, just *keep moving forward*.
                Tile::Empty => {},
                // For the edged mirrors, edge ourselves
                Tile::MirrorRight => {
                    self.direction = self.direction.reflected_right();
                },
                Tile::MirrorLeft => {
                    self.direction = self.direction.reflected_left();
                },

                // If we find a vertical bar and we're going horizontally, or vice versa,
                // explore the two directions the bar is pointing at. Then break ourselves. Ouch
                Tile::VerticalBar => {
                    if self.direction.is_horizontal() {
                        self.with_direction(Direction::Up).drive_with_points(layout, explored_beams);
                        self.with_direction(Direction::Down).drive_with_points(layout, explored_beams);
                        break;
                    }
                },
                Tile::HorizontalBar => {
                    if self.direction.is_vertical() {
                        self.with_direction(Direction::Left).drive_with_points(layout, explored_beams);
                        self.with_direction(Direction::Right).drive_with_points(layout, explored_beams);
                        break;
                    }
                }
            }

            // Move self to next position; if invalid, break
            if self.move_in(self.direction, layout).is_none() {
                break;
            }
        }
    }

    fn move_in(&mut self, direction: Direction, layout: &Layout) -> Option<()> {
        match direction {
            Direction::Up => self.y = self.y.checked_sub(1)?,
            Direction::Down => self.y = self.y.checked_add(1)?,
            Direction::Left => self.x = self.x.checked_sub(1)?,
            Direction::Right => self.x = self.x.checked_add(1)?,
        }

        if self.x >= layout.width() || self.y >= layout.height() {
            return None;
        }

        Some(())
    }
}

impl Layout {
    pub fn parse(input: &str) -> Option<Self> {
        let mut rows = Vec::new();

        for line in input.lines() {
            rows.push(line.chars().map(|c| Tile::from_char(c).unwrap()).collect());
        }

        Some(Self {
            rows,
        })
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get_tile_at_beam(&self, point: &Beam) -> Tile {
        self.rows[point.y][point.x]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    VerticalBar,
    HorizontalBar
}

impl Tile {
    pub fn from_char(c: char) -> Option<Tile> {
        let tile = match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorRight,
            '\\' => Tile::MirrorLeft,
            '|' => Tile::VerticalBar,
            '-' => Tile::HorizontalBar,
            _ => return None
        };

        Some(tile)
    }
}