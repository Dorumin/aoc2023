use std::{ops::Range, collections::HashSet, fmt::{Display, Write}, cell::RefCell, time::Instant};

#[derive(Debug)]
pub struct Map {
    pub rows: Vec<Row>
}

#[derive(Debug)]
pub struct Row {
    index: usize,
    pub records: Vec<Record>,
    pub groups: Vec<usize>
}

impl Map {
    pub fn parse(input: &str) -> Option<Self> {
        let rows = input.lines()
            .enumerate()
            .map(|(index, line)| {
                Row::parse_line(line, index).unwrap()
            })
            .collect();

        Some(Self {
            rows
        })
    }

    pub fn unfold(&mut self) {
        for row in self.rows.iter_mut() {
            row.records = std::iter::repeat(()).take(5).enumerate().flat_map(|(i, _)| {
                let mut cloned = row.records.clone();

                if i != 4 {
                    cloned.push(Record::Unknown);
                }

                cloned
            }).collect();
            row.groups = std::iter::repeat(()).take(5).flat_map(|_| row.groups.clone()).collect();
        }
    }
}

impl Row {
    pub fn parse_line(input: &str, index: usize) -> Option<Self> {
        let (records, groups) = input.split_once(' ')?;
        let records = records.chars()
            .map(|c| Record::from_char(c).unwrap())
            .collect();
        let groups = groups.split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Some(Self {
            index,
            records,
            groups
        })
    }


    pub fn possible_arrangements(&self) -> Vec<Vec<Record>> {
        let possibility_buffer: Vec<RecordCompute> = self.records.iter().cloned().map(From::from).collect();

        let mut possibilities = HashSet::new();

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

        fn explore_group(
            ordered_groups: &[usize],
            possibilities: &mut HashSet<Vec<Record>>,
            possibility_buffer: &[RecordCompute],
            group_index: usize,
            start_search: usize
        ) {
            if group_index >= ordered_groups.len() {
                // Reached the end
                if possibility_buffer.iter().any(|r| matches!(r, RecordCompute::Damaged)) {
                    dbg!(possibility_buffer);
                    panic!();
                }

                if !possibility_buffer.iter().any(|r| matches!(r, RecordCompute::Damaged)) {
                    let possibility = possibility_buffer.iter()
                        .cloned()
                        .map(|r| Record::try_from(r).unwrap())
                        .collect();

                    if possibilities.contains(&possibility) {
                        panic!("duplicate possibility");
                    }

                    possibilities.insert(possibility);
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

                    explore_group(
                        ordered_groups,
                        possibilities,
                        &new_buffer,
                        group_index + 1,
                        start + group_length + 1
                    );
                }

                if let Some(before_index) = start.checked_sub(1) {
                    if possibility_buffer[before_index] == RecordCompute::Damaged {
                        break;
                    }
                }
            }
        }

        explore_group(
            &self.groups,
            &mut possibilities,
            &possibility_buffer,
             0,
             0,
        );

        println!("computed one row {}", possibilities.len());

        possibilities.into_iter().collect()
    }

    pub fn possible_arrangements_count(&self) -> u64 {
        // The possibility buffers needs an extra one at the end
        // because we copy the previous one into the next when moving to the next group
        let mut possibility_buffers: Vec<RefCell<Vec<RecordCompute>>> = Vec::new();

        possibility_buffers.resize_with(
            self.groups.len() + 1,
            || RefCell::new(self.records.iter().cloned().map(From::from).collect())
        );

        let mut possibility_count = 0;
        let start_time = Instant::now();

        fn is_range_damageable(possibility_buffer: &[RecordCompute], range: &Range<usize>) -> bool {
            debug_assert!(range.end <= possibility_buffer.len());

            let before = range.start.checked_sub(1).map(|i| &possibility_buffer[i]);
            let after = possibility_buffer.get(range.end); // Ranges are non-exclusive at the end
            let slice = &possibility_buffer[range.clone()];

            !matches!(before, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor))
            && !matches!(after, Some(RecordCompute::Damaged | RecordCompute::DamagedAccountedFor))
            && slice.iter().all(|record| matches!(record, RecordCompute::Damaged | RecordCompute::Unknown))
        }

        fn explore_group(
            ordered_groups: &[usize],
            possibility_count: &mut u64,
            possibility_buffers: &[RefCell<Vec<RecordCompute>>],
            group_index: usize,
            start_search: usize
        ) {
            let possibility_buffer = &possibility_buffers[group_index].borrow();

            if group_index >= ordered_groups.len() {
                // Reached the end

                // If there are any damaged slots left unaccounted for, we fucked up
                if possibility_buffer.iter().rev().any(|r| matches!(r, RecordCompute::Damaged)) {
                    return;
                }

                *possibility_count += 1;

                if (*possibility_count % 100000) == 0 {
                    print!("\r{possibility_count}");
                }

                return;
            }

            let group_length = ordered_groups[group_index];
            let remaining_groups = &ordered_groups[group_index..];
            let remaining_group_lengths = remaining_groups.iter()
                // We add +2 to each group size to account for surrounding operational records
                // This has to be counterbalanced by a total + 2 to account for the limits of the line
                // being able to harbor groups, regardless of surrounding machinery
                // plus one, of course. Why is there always a plus one
                .fold(0, |sum, g| sum + *g + 2);

            for start in start_search..(possibility_buffer.len() - remaining_group_lengths + remaining_groups.len() + 2) {
                let range = start..start + group_length;

                if is_range_damageable(possibility_buffer, &range) {
                    // let indent = " ".repeat(group_index * 4);
                    // eprintln!("{indent}{group_index} damageable at range {}..{}", range.start, range.end);

                    let mut next_buffer = possibility_buffers[group_index + 1].borrow_mut();
                    next_buffer.copy_from_slice(possibility_buffer);

                    if let Some(before_index) = range.start.checked_sub(1) {
                        next_buffer[before_index] = RecordCompute::Operational;
                    }

                    if let Some(after_item) = next_buffer.get_mut(range.end) {
                        *after_item = RecordCompute::Operational;
                    }

                    next_buffer[range].fill(RecordCompute::DamagedAccountedFor);

                    drop(next_buffer);

                    explore_group(
                        ordered_groups,
                        possibility_count,
                        possibility_buffers,
                        group_index + 1,
                        start + group_length + 1
                    );
                }

                if let Some(before_index) = start.checked_sub(1) {
                    if possibility_buffer[before_index] == RecordCompute::Damaged {
                        break;
                    }
                }
            }
        }

        explore_group(
            &self.groups,
            &mut possibility_count,
            &possibility_buffers,
             0,
             0,
        );


        println!("\r{}. row count {possibility_count} in {:?}", self.index + 1, start_time.elapsed());

        possibility_count
    }

    pub fn possible_arrangements_starts(&self) -> Vec<heapless::Vec<u8, 32>> {
        let possibility_buffer: Vec<RecordCompute> = self.records.iter().cloned().map(From::from).collect();
        let mut stacked_starts = heapless::Vec::new();

        let mut possibilities = HashSet::new();

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

        fn explore_group(
            ordered_groups: &[usize],
            possibilities: &mut HashSet<heapless::Vec<u8, 32>>,
            stacked_starts: &mut heapless::Vec<u8, 32>,
            possibility_buffer: &[RecordCompute],
            group_index: usize,
            start_search: usize
        ) {
            if group_index >= ordered_groups.len() {
                // Reached the end
                if !possibility_buffer.iter().any(|r| matches!(r, RecordCompute::Damaged)) {
                    possibilities.insert(stacked_starts.clone());
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

                    stacked_starts.push(start.try_into().unwrap()).unwrap();

                    explore_group(
                        ordered_groups,
                        possibilities,
                        stacked_starts,
                        &new_buffer,
                        group_index + 1,
                        start + group_length + 1
                    );

                    stacked_starts.pop().unwrap();
                }

                if let Some(before_index) = start.checked_sub(1) {
                    if possibility_buffer[before_index] == RecordCompute::Damaged {
                        break;
                    }
                }
            }
        }

        explore_group(
            &self.groups,
            &mut possibilities,
            &mut stacked_starts,
            &possibility_buffer,
             0,
             0,
        );

        println!("computed one row {}", possibilities.len());

        possibilities.into_iter().collect()
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for record in self.records.iter() {
            f.write_char(record.to_char())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecordCompute {
    Operational,
    Damaged,
    DamagedAccountedFor,
    Unknown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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