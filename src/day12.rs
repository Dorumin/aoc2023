use std::ops::Range;

#[derive(Debug)]
pub struct Map {
    pub rows: Vec<Row>
}

#[derive(Debug)]
pub struct Row {
    pub records: Vec<Record>,
    pub groups: Vec<usize>
}

impl Map {
    pub fn parse(input: &str) -> Option<Self> {
        let rows = input.lines()
            .map(|input| {
                Row::parse_line(input).unwrap()
            })
            .collect();

        Some(Self {
            rows
        })
    }
}

impl Row {
    pub fn parse_line(input: &str) -> Option<Self> {
        let (records, groups) = input.split_once(' ')?;
        let records = records.chars()
            .map(|c| Record::from_char(c).unwrap())
            .collect();
        let groups = groups.split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Some(Self {
            records,
            groups
        })
    }

    pub fn possible_arrangements(&self) -> Vec<Vec<Record>> {
        let possibility_buffer: Vec<RecordCompute> = self.records.iter().cloned().map(From::from).collect();

        let mut possibilities = vec![];

        fn is_range_damageable(possibility_buffer: &[RecordCompute], range: &Range<usize>) -> bool {
            if range.end > possibility_buffer.len() {
                return false;
            }

            let before = range.start.checked_sub(1).map(|i| &possibility_buffer[i]);
            let after = possibility_buffer.get(range.end); // Ranges are non-exclusive at the end
            let slice = &possibility_buffer[range.clone()];

            // if range == &(5..7) {
            //     dbg!(range);
            //     dbg!(!matches!(before, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor)));
            //     dbg!(!matches!(after, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor)));
            //     dbg!(slice.iter().all(|record| matches!(record, RecordCompute::Damaged | RecordCompute::Unknown)));
            //     dbg!(slice);
            // }

            !matches!(before, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor))
            && !matches!(after, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor))
            && slice.iter().all(|record| matches!(record, RecordCompute::Damaged | RecordCompute::Unknown))
        }

        fn explore_group(ordered_groups: &[usize], possibility_buffer: &[RecordCompute], possibilities: &mut Vec<Vec<Record>>, group_index: usize, start_search: usize) {
            if group_index >= ordered_groups.len() {
                if !possibility_buffer.iter().any(|r| matches!(r, RecordCompute::Damaged)) {
                    let possibility = possibility_buffer.iter()
                        .cloned()
                        .map(|r| Record::try_from(r).unwrap())
                        .collect();

                    if !possibilities.contains(&possibility) {
                        possibilities.push(possibility);
                    }
                }

                return;
            }

            let group_length = ordered_groups[group_index];

            for start in start_search..possibility_buffer.len() {
                let range = start..start + group_length;

                if is_range_damageable(possibility_buffer, &range) {
                    // let indent = " ".repeat(group_index * 4);
                    // eprintln!("{indent}{group_index} damageable at range {}..{}", range.start, range.end);

                    let mut new_buffer = possibility_buffer.to_owned();

                    if let Some(before_index) = range.start.checked_sub(1) {
                        new_buffer[before_index] = RecordCompute::Operational;
                    }

                    if let Some(after_item) = new_buffer.get_mut(range.end) {
                        *after_item = RecordCompute::Operational;
                    }

                    new_buffer[range].fill(RecordCompute::DamagedAccountedFor);

                    explore_group(ordered_groups, &new_buffer, possibilities, group_index + 1, start);
                }
            }
        }

        explore_group(&self.groups, &possibility_buffer, &mut possibilities, 0, 0);

        possibilities
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordCompute {
    Operational,
    Damaged,
    DamagedAccountedFor,
    Unknown
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Record {
    Operational,
    Damaged,
    Unknown
}

impl Record {
    pub fn from_char(c: char) -> Option<Record> {
        let record = match c {
            '.' => Record::Operational,
            '#' => Record::Damaged,
            '?' => Record::Unknown,
            _ => return None
        };

        Some(record)
    }

    pub fn to_char(&self) -> char {
        match self {
            Record::Operational => '.',
            Record::Damaged => '#',
            Record::Unknown => '?'
        }
    }
}

impl From<Record> for RecordCompute {
    fn from(record: Record) -> Self {
        match record {
            Record::Operational => RecordCompute::Operational,
            Record::Damaged => RecordCompute::Damaged,
            Record::Unknown => RecordCompute::Unknown
        }
    }
}

impl TryFrom<RecordCompute> for Record {
    type Error = ();

    fn try_from(record_compute: RecordCompute) -> Result<Self, Self::Error> {
        let record = match record_compute {
            RecordCompute::Operational => Self::Operational,
            RecordCompute::DamagedAccountedFor => Self::Damaged,

            // RecordComputes when converted are known to have their unknown variants
            // are actually operational
            RecordCompute::Unknown => Self::Operational,

            // This case should never be tried to be converted into a Record
            RecordCompute::Damaged => return Err(()),
        };

        Ok(record)
    }
}