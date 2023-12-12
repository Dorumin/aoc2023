use anyhow::Context;
use itertools::Itertools;

use std::{collections::HashMap, ops::Range};

type Seed = (MappingType, u64);
type SeedRange = (MappingType, Range<u64>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MappingType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl MappingType {
    fn from_str(type_str: &str) -> Option<MappingType> {
        let mapping_type = match type_str {
            "seed" => MappingType::Seed,
            "soil" => MappingType::Soil,
            "fertilizer" => MappingType::Fertilizer,
            "water" => MappingType::Water,
            "light" => MappingType::Light,
            "temperature" => MappingType::Temperature,
            "humidity" => MappingType::Humidity,
            "location" => MappingType::Location,
            _ => return None
        };

        Some(mapping_type)
    }
}

#[derive(Debug)]
pub struct SeedsMap {
    seed_numbers: Vec<u64>,
    pub mappings: HashMap<MappingType, Mapping>
}

#[derive(Debug)]
pub struct Mapping {
    from: MappingType,
    to: MappingType,
    pub lines: Vec<MappingLine>
}

#[derive(Debug)]
pub struct MappingLine {
    numbers: Vec<u64>
}

impl SeedsMap {
    pub fn transform_seeds(&self) -> Vec<Seed> {
        self.seeds().into_iter()
            .map(|seed| self.transform_seed(seed))
            .collect()
    }

    pub fn transform_seed_ranges(&self) -> Vec<SeedRange> {
        dbg!(self.mappings.keys().collect_vec());
        self.seeds_ranges().iter()
            .flat_map(|range| self.transform_seed_range(range))
            .collect()
    }

    pub fn transform_seed_range(&self, range: &SeedRange) -> Vec<SeedRange> {
        let mut ranges = vec![range.clone()];
        let mut transformed_ranges = vec![];

        'range_loop:
        while let Some(mut range) = ranges.pop() {
            if !self.mappings.contains_key(&range.0) {
                // dbg!(&range.0);
                transformed_ranges.push(range);
                continue;
            }

            let current_mapping = &self.mappings[&range.0];

            for line in current_mapping.lines.iter() {
                let (offset_range, rest) = line.offset_range(range.1.clone());

                if let Some(offset_range) = offset_range {
                    ranges.push((current_mapping.to, offset_range));

                    for unmapped in rest {
                        ranges.push((range.0, unmapped));
                    }

                    continue 'range_loop;
                }
            }

            // eprintln!("passthrough {range:?}");
            // This only runs if no line matched this range
            range.0 = current_mapping.to;
            ranges.push(range);
        }

        transformed_ranges
    }

    pub fn seeds(&self) -> Vec<Seed> {
        self.seed_numbers.iter().map(|n| (MappingType::Seed, *n)).collect()
    }

    pub fn seeds_ranges(&self) -> Vec<SeedRange> {
        self.seed_numbers.chunks_exact(2)
            .map(|s| (MappingType::Seed, s[0]..(s[0] + s[1])))
            .collect()
    }

    pub fn transform_seed(&self, seed: Seed) -> Seed {
        let mut seed = seed;

        loop {
            if !self.mappings.contains_key(&seed.0) {
                break;
            }

            let current_mapping = &self.mappings[&seed.0];

            for line in current_mapping.lines.iter() {
                if line.source_range().contains(&seed.1) {
                    seed = (current_mapping.to, seed.1 - line.source_start() + line.destination_start());
                    break;
                }
            }

            seed.0 = current_mapping.to;
        }

        seed
    }

    pub fn parse(input: &str) -> anyhow::Result<SeedsMap> {
        let mut lines = input.lines();
        let seeds_line = lines.next().context("there must be a first line")?;
        let seeds_line = seeds_line.strip_prefix("seeds: ").context("first line must start with seeds")?;
        let seed_numbers: Vec<_> = seeds_line.split_ascii_whitespace().map(|n| n.parse().context("seeds must be u64s").unwrap()).collect();

        assert_eq!(lines.next(), Some(""));

        let mut mappings = HashMap::new();

        let mut mapping: Option<Mapping> = None;

        for line in lines {
            if line.is_empty() {
                assert!(mapping.is_some());

                if let Some(mapping) = mapping.take() {
                    mappings.insert(mapping.from, mapping);
                }

                mapping = None;
                continue;
            } else if let Some(mapping) = mapping.as_mut() {
                mapping.insert_line(line);
                continue;
            }

            mapping = Some(Mapping::from_header_line(line).unwrap());
        }

        if let Some(mapping) = mapping.take() {
            mappings.insert(mapping.from, mapping);
        }

        Ok(SeedsMap {
            seed_numbers,
            mappings
        })
    }
}

impl Mapping {
    fn from_header_line(line: &str) -> anyhow::Result<Mapping> {
        let line = line.strip_suffix(" map:").context("mapping header must end with map:")?;
        let mut parts = line.split('-');

        let from = parts.next().and_then(MappingType::from_str).context("From type must be a valid mapping type")?;
        assert_eq!(parts.next(), Some("to"));
        let to = parts.next().and_then(MappingType::from_str).context("To type must be a valid mapping type")?;

        Ok(Mapping {
            from,
            to,
            lines: Vec::new(),
        })
    }

    fn insert_line(&mut self, line: &str) {
        self.lines.push(MappingLine::parse_line(line));
    }
}

impl MappingLine {
    pub fn parse_line(line: &str) -> Self {
        let numbers = line.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            numbers
        }
    }

    fn range_size(&self) -> u64 {
        self.numbers[2]
    }

    fn source_start(&self) -> u64 {
        self.numbers[1]
    }

    fn destination_start(&self) -> u64 {
        self.numbers[0]
    }

    pub fn source_range(&self) -> Range<u64> {
        self.source_start()..(self.source_start() + self.range_size())
    }

    pub fn destination_range(&self) -> Range<u64> {
        self.destination_start()..(self.destination_start() + self.range_size())
    }

    pub fn offset(&self, index: u64) -> u64 {
        index - self.source_start() + self.destination_start()
    }

    pub fn offset_range(&self, range: Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        let get_range = |range: Range<u64>| -> (Option<Range<u64>>, Vec<Range<u64>>) {
            let source = self.source_range();

            if source.start >= range.end || source.end <= range.start {
                return (None, vec![range.start..range.end]);
            }

            if source.contains(&range.start) && source.contains(&range.end) {
                return (Some(range), vec![]);
            }

            if source.contains(&range.start) {
                return (Some(range.start..source.end), vec![source.end..range.end])
            }

            if source.contains(&range.end) {
                return (Some(source.start..range.end), vec![range.start..source.start])
            }

            (Some(source.start..source.end), vec![range.start..source.start, source.end..range.end])
        };

        let (mapped, rest) = get_range(range);
        let mapped = mapped.map(|r| self.offset(r.start)..self.offset(r.end));

        (mapped, rest)
    }
}