use std::{time::Instant, collections::HashMap};

use aoc2023::day14::Platform;
use itertools::Itertools;
use num::Integer;

const INPUT: &str = include_str!("../inputs/day14-platform.txt");

fn main() {
    let mut platform = Platform::parse(INPUT).unwrap();

    let mut cache: HashMap<Platform, i32> = HashMap::new();

    let start = Instant::now();
    let max_len = 1000000000;
    let mut i = 0;
    let mut jumped = true;
    while i < max_len {
        // Only run this code path once
        if let Some(cached) = jumped.then(|| cache.get(&platform)).flatten() {
            let interval = i - cached;
            let (jump_size, remainder) = (max_len - i).div_rem(&interval);

            dbg!(cached, i, interval, jump_size, remainder);

            // Jump ahead to the position for the next index, minus one so we get exactly `remainder` more iteration
            i += jump_size * interval - 1;
            jumped = false;

            // Debug stuff
            for (platform, index) in cache.iter().sorted_by_key(|p| p.1) {
                println!("{} {}", index, platform.total_load());
            }

            println!("Jumped to {i} with {remainder} remaining");
        } else {
            cache.insert(platform.clone(), i);
            platform.tilt_cycle();
        }

        i += 1;
    }
    eprintln!("{:?}", start.elapsed());

    // println!("{}", platform);

    dbg!(platform.total_load());
}