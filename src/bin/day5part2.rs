use aoc2023::day5::{SeedsMap, MappingType};

const INPUT: &str = include_str!("../inputs/day5-seeds.txt");

fn main() {
    let map = SeedsMap::parse(INPUT).unwrap();

    // Verify ranges within a mapping group are non overlapping
    map.mappings.values().for_each(|mapping| {
        mapping.lines.iter().for_each(|line| {
            let range = line.source_range();

            if let Some(outer) = mapping.lines.iter().find(|outer|
                outer.source_range().start > range.end && outer.source_range().end < range.start
            ) {
                panic!("overlapping!!! {range:?} {outer:?}");
            }
        });
    });

    // let line = MappingLine::parse_line("20 10 10");

    // dbg!(line.source_range());
    // dbg!(line.destination_range());
    // dbg!(line.source_range().contains(&11));
    // dbg!(line.source_range().contains(&15));
    // dbg!(line.offset(11));
    // dbg!(line.offset(15));

    let binding = map.transform_seed_ranges();
    let transformed_ranges: Vec<_> = binding.iter().collect();

    // dbg!(&transformed_ranges);

    dbg!(transformed_ranges.iter().map(|range| range.1.start).min());
    dbg!(transformed_ranges.iter().all(|r| r.0 == MappingType::Location));
    // dbg!(map.transform_seed_ranges().iter().map(|range| range.1.start).count());
}