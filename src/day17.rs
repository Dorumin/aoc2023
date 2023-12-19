use pathfinding::directed::astar;

pub use crate::common::Direction;

pub struct Crucible {
    pub rows: Vec<Vec<u64>>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    x: usize,
    y: usize,
    pub heat: u64,
    direction: Direction,
    consecutive_straight_steps: u64
}

impl Crucible {
    pub fn parse(input: &str) -> Option<Self> {
        let mut rows = Vec::new();

        for line in input.lines() {
            let row = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

            rows.push(row);
        }

        Some(Self {
            rows
        })
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get_position(&self, x: usize, y: usize, consecutive_straight_steps: u64, direction: Direction) -> Position {
        Position {
            x,
            y,
            heat: self.rows[y][x],
            consecutive_straight_steps,
            direction
        }
    }

    pub fn print_path(&self, path: &[Position]) {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, heat) in row.iter().enumerate() {
                if let Some(p) = path.iter().find(|p| p.x == x && p.y == y) {
                    let c = match p.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>'
                    };

                    print!("{c}");
                } else {
                    print!("{heat}");
                }
            }

            println!();
        }
    }
}

impl Position {
    fn next(&self, direction: Direction, min_straight_steps: u64, max_straight_steps: u64, crucible: &Crucible) -> Option<Position> {
        // No takesies backsies
        if !direction == self.direction {
            return None;
        }

        let mut next_x = self.x;
        let mut next_y = self.y;

        let consecutive_straight_steps = if direction == self.direction {
            self.consecutive_straight_steps + 1
        } else {
            1
        };

        if consecutive_straight_steps > max_straight_steps {
            return None;
        }

        if self.consecutive_straight_steps > 0 && self.consecutive_straight_steps < min_straight_steps && direction != self.direction {
            return None;
        }

        // eprintln!("{} {} {consecutive_straight_steps} {:?} {:?}", self.x, self.y, direction, self.direction);

        match direction {
            Direction::Up => next_y = self.y.checked_sub(1)?,
            Direction::Down => next_y = self.y + 1,
            Direction::Left => next_x = self.x.checked_sub(1)?,
            Direction::Right => next_x = self.x + 1,
        }

        if next_x >= crucible.width() || next_y >= crucible.height() {
            return None;
        }

        let next = crucible.get_position(next_x, next_y, consecutive_straight_steps, direction);

        Some(next)
    }

    fn adjacents(&self, min_straight_steps: u64, max_straight_steps: u64, crucible: &Crucible) -> Vec<Position> {
        let mut adjacents = Vec::with_capacity(4);

        self.next(Direction::Up, min_straight_steps, max_straight_steps, crucible).into_iter().for_each(|p| adjacents.push(p));
        self.next(Direction::Down, min_straight_steps, max_straight_steps, crucible).into_iter().for_each(|p| adjacents.push(p));
        self.next(Direction::Left, min_straight_steps, max_straight_steps, crucible).into_iter().for_each(|p| adjacents.push(p));
        self.next(Direction::Right, min_straight_steps, max_straight_steps, crucible).into_iter().for_each(|p| adjacents.push(p));

        // eprintln!("adj for ({}, {}): {:?}", self.x, self.y, &adjacents);

        // Make sure adjacents does not include the end point if its consecutive steps is < than min_straight_steps
        adjacents.retain(|p| {
            let is_end = p.x == crucible.width() - 1 && p.y == crucible.height() - 1;
            if !is_end {
                return true;
            }

            // dbg!(p);

            p.consecutive_straight_steps >= min_straight_steps
        });

        adjacents
    }

    pub fn pathfind_to(&self, end_point: &Position, crucible: &Crucible, min_straight_steps: u64, max_straight_steps: u64) -> Vec<Position> {
        let results = astar::astar(
            self,
            |p| p.adjacents(min_straight_steps, max_straight_steps, crucible).into_iter().map(|p| {
                let heat = p.heat;
                (p, heat)
            }),
            |p| farthattan_distance(p, end_point).try_into().unwrap(),
            |p| p.x == end_point.x && p.y == end_point.y
        );

        let mut path = results.unwrap().0;

        // Remove first element from path
        path.remove(0);

        path
    }
}

// Puxutilities
pub fn farthattan_distance(a: &Position, b: &Position) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}