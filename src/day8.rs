use std::{collections::{HashMap, hash_map::Keys}, fmt::Debug, ops::Index};

use once_cell::sync::Lazy;
use regex::Regex;

const START_LOCATION: Location = Location(*b"AAA");
const END_LOCATION: Location = Location(*b"ZZZ");

#[derive(Debug)]
pub struct Map {
    directions: Vec<Direction>,
    locations: HashMap<Location, (Location, Location)>
}

impl Map {
    pub fn locations(&self) -> Keys<'_, Location, (Location, Location)> {
        self.locations.keys()
    }

    pub fn parse(input: &str) -> Option<Map> {
        static NODE_LINE_REGEX: Lazy<Regex> = Lazy::new(||
            Regex::new(r"^(?<start>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)$").unwrap()
        );

        let mut lines = input.lines();
        let directions = lines.next()?.chars().map(|c| Direction::from_char(c).unwrap()).collect();

        let mut locations = HashMap::new();

        assert_eq!(lines.next(), Some(""));

        for line in lines {
            let caps = NODE_LINE_REGEX.captures(line)?;

            let start = Location::from_str(caps.name("start")?.as_str())?;
            let left = Location::from_str(caps.name("left")?.as_str())?;
            let right = Location::from_str(caps.name("right")?.as_str())?;

            locations.insert(start, (left, right));
        }

        Some(Map {
            directions,
            locations,
        })
    }

    pub fn walk(&self) -> Vec<Location> {
        self.walk_from_until(START_LOCATION, |loc| loc == END_LOCATION)
    }

    pub fn step_node(&self, location: Location, direction: Direction) -> Location {
        return self.locations[&location][direction];
    }

    pub fn directions_infinite(&self) -> impl Iterator<Item = Direction> + '_ {
        (0..self.directions.len()).cycle().map(|i| self.directions[i])
    }

    pub fn walk_from_until<F>(&self, start_location: Location, end_condition: F) -> Vec<Location>
    where
        F: Fn(Location) -> bool
    {
        // I would love to make this a generator
        let mut locations = vec![];
        let mut current_location = start_location;

        for i in (0..self.directions.len()).cycle() {
            let direction = &self.directions[i];
            current_location = self.locations[&current_location][*direction];

            locations.push(current_location);

            if end_condition(current_location) {
                break;
            }
        }

        locations
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right
}

impl Direction {
    fn from_char(dir: char) -> Option<Direction> {
        let direction = match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return None
        };

        Some(direction)
    }
}

impl Index<Direction> for (Location, Location) {
    type Output = Location;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Left => &self.0,
            Direction::Right => &self.1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Location([u8; 3]);

impl Location {
    fn from_str(three_chars: &str) -> Option<Location> {
        let location = Location(three_chars.as_bytes().try_into().ok()?);
        Some(location)
    }

    pub fn is_ghost_start(&self) -> bool {
        self.0[2] == b'A'
    }

    pub fn is_ghost_end(&self) -> bool {
        self.0[2] == b'Z'
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", std::str::from_utf8(&self.0).unwrap()))
    }
}