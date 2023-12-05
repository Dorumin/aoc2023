use anyhow::Context;

use std::{collections::HashMap, ops::Range};

type Seed = u64;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MappingType {
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
    seeds: Vec<Seed>,
    mappings: HashMap<MappingType, Mapping>
}

#[derive(Debug)]
pub struct Mapping {
    from: MappingType,
    to: MappingType,
    lines: Vec<MappingLine>
}

#[derive(Debug)]
pub struct MappingLine {
    numbers: Vec<Seed>
}

impl SeedsMap {
    pub fn transform_seeds(&self) -> Vec<Seed> {
        self.seeds.iter()
            .cloned()
            .map(|seed| self.transform_seed(seed))
            .collect()
    }

    pub fn transform_seed(&self, seed: Seed) -> Seed {
        let mut current_mapping = &self.mappings[&MappingType::Seed];
        let mut seed = seed;

        loop {
            for line in current_mapping.lines.iter() {
                if line.source_range().contains(&seed) {
                    seed = seed - line.source_start() + line.destination_start();
                    break;
                }
            }

            if self.mappings.contains_key(&current_mapping.to) {
                current_mapping = &self.mappings[&current_mapping.to];
            } else {
                break;
            }
        }

        seed
    }

    pub fn parse(input: &str) -> anyhow::Result<SeedsMap> {
        let mut lines = input.lines();
        let seeds_line = lines.next().context("there must be a first line")?;
        let seeds_line = seeds_line.strip_prefix("seeds: ").context("first line must start with seeds")?;
        let seeds: Vec<_> = seeds_line.split_ascii_whitespace().map(|n| n.parse().context("seeds must be u64s").unwrap()).collect();

        assert_eq!(lines.next(), Some(""));

        let mut mappings = HashMap::new();

        let mut mapping: Option<Mapping> = None;

        for line in lines {
            if line == "" {
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


        Ok(SeedsMap {
            seeds,
            mappings
        })
    }
}

impl Mapping {
    fn from_header_line(line: &str) -> anyhow::Result<Mapping> {
        let line = line.strip_suffix(" map:").context("mapping header must end with map:")?;
        let mut parts = line.split('-');

        let from = parts.next().and_then(|p| MappingType::from_str(p)).context("From type must be a valid mapping type")?;
        assert_eq!(parts.next(), Some("to"));
        let to = parts.next().and_then(|p| MappingType::from_str(p)).context("To type must be a valid mapping type")?;

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
    fn parse_line(line: &str) -> Self {
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

    fn source_range(&self) -> Range<u64> {
        self.source_start()..(self.source_start() + self.range_size())
    }

    pub fn destination_range(&self) -> Range<u64> {
        self.destination_start()..(self.destination_start() + self.range_size())
    }
}