
#[derive(Debug)]
pub struct Patterns {
    pub patterns: Vec<Pattern>
}

impl Patterns {
    pub fn parse(input: &str) -> Option<Self> {
        let mut patterns = Vec::new();
        let mut pattern = None;

        for line in input.lines() {
            if line.is_empty() {
                patterns.push(pattern.take().unwrap());
                continue;
            }

            let pat = pattern.get_or_insert_with(Pattern::default);

            pat.rows.push(Row::from_line(line).unwrap());
        }

        if let Some(pat) = pattern {
            patterns.push(pat);
        }

        Some(Self {
            patterns
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct Pattern {
    rows: Vec<Row>
}

impl Pattern {
    fn parse(grid: &str) -> Option<Self> {
        let rows = grid.lines().map(|line| Row::from_line(line).unwrap()).collect();

        Some(Self {
            rows
        })
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn width(&self) -> usize {
        self.rows.first().unwrap().cells.len()
    }

    pub fn smudges(&self) -> impl Iterator<Item = Pattern> + '_ {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |_| y).enumerate())
            .map(move |(x, y)| {
                let mut cloned: Pattern = self.clone();

                cloned.rows[y].cells[x] = cloned.rows[y].cells[x].other();

                cloned
            })
    }

    pub fn find_mirror_axis(&self, filter: impl Fn(usize, usize) -> bool) -> (Option<usize>, Option<usize>) {
        let width = self.width();
        let height = self.height();

        let mut y_mirror = None;
        let mut x_mirror = None;

        'outer_loop:
        for mid_x in 0..width - 1 {
            for row in self.rows.iter() {
                for i in 0..width {
                    // eprintln!("{:?} {:?}", (mid_x + 1).checked_sub(i), Some(mid_x + i));

                    // If left is negative, or right is beyond the end of the row, that is considered OK
                    let Some(left_cell) = (mid_x + 1).checked_sub(i).and_then(|i| row.cells.get(i)) else {
                        break;
                    };
                    let Some(right_cell) = Some(mid_x + i).and_then(|i| row.cells.get(i)) else {
                        break;
                    };

                    if left_cell != right_cell {
                        continue 'outer_loop;
                    }
                }
            }

            if filter(0, mid_x) {
                x_mirror = Some(mid_x);
                break;
            }
        }

        'outer_loop:
        for mid_y in 0..height - 1 {
            for i in 0..height {
                // eprintln!("{:?} {:?}", (mid_y + 1).checked_sub(i), Some(mid_y + i));

                // If left is negative, or right is beyond the end of the row, that is considered OK
                let Some(top_row) = (mid_y + 1).checked_sub(i).and_then(|i| self.rows.get(i)) else {
                    break;
                };
                let Some(bottom_row) = Some(mid_y + i).and_then(|i| self.rows.get(i)) else {
                    break;
                };

                if top_row != bottom_row {
                    continue 'outer_loop;
                }
            }

            if filter(1, mid_y) {
                y_mirror = Some(mid_y);
                break;
            }
        }

        (x_mirror, y_mirror)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    cells: Vec<Cell>
}

impl Row {
    fn from_line(line: &str) -> Option<Self> {
        let cells = line.chars().map(|c| Cell::from_char(c).unwrap()).collect();

        Some(Self {
            cells
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Ash,
    Rock
}

impl Cell {
    fn from_char(c: char) -> Option<Self> {
        let c = match c {
            '.' => Cell::Ash,
            '#' => Cell::Rock,
            _ => return None
        };


        Some(c)
    }

    fn other(&self) -> Self {
        match self {
            Cell::Ash => Cell::Rock,
            Cell::Rock => Cell::Ash,
        }
    }
}