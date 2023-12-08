use aoc2023::day8::Map;
use num::integer::lcm;

const INPUT: &str = include_str!("../inputs/day8-dromedary.txt");

fn main() {
    let map = Map::parse(INPUT).unwrap();

    let mut ghost_locations: Vec<_> = map.locations().cloned().filter(|loc| loc.is_ghost_start()).collect();

    let collected: Vec<_> = ghost_locations.iter().map(|start|
        map.walk_from_until(*start, |loc| loc.is_ghost_end()).len() as u64
    ).collect();


    let closest = collected.iter().fold(1, |lowest, n| lcm(lowest, *n));

    dbg!(closest);

    // for direction in map.directions_infinite() {
    //     // dbg!(direction);

    //     for location in ghost_locations.iter_mut() {
    //         *location = map.step_node(*location, direction);
    //     }

    //     if ghost_locations.iter().all(|loc| loc.is_ghost_end()) {
    //         break;
    //     }
    // }

    // dbg!(ghost_locations);

    // let total: u64 = start_locations
    //     .map(|start|
    //         map.walk_from_until(*start, |loc| loc.is_ghost_end()).len() as u64
    //     )
    //     .sum();

    // dbg!(total);
}