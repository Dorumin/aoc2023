use std::time::Instant;

use aoc2023::day10::Map;
use geo::{LineString, Coord, Polygon, Contains, Point};

const INPUT: &str = include_str!("../inputs/day10-mario.txt");

fn main() {
    let map = Map::parse(INPUT).unwrap();

    println!("{width} {height}", width = map.width, height = map.height);
    // println!("{map}");

    let mut path = map.follow(map.start()).unwrap();

    // println!("{}", map.display_path(&path));

    // Make the path closed
    path.push(map.start());

    let linestr = time("build linestr", || LineString::new(
        path.iter()
            .map(|p| Coord { x: p.x as f64, y: p.y as f64 })
            .collect()
    ));

    assert!(linestr.is_closed());

    let shape = time("build polygon", || Polygon::new(linestr, vec![]));

    let inner_count = time("count of contains", || {
        map.positions().into_iter()
            .filter(|&pos|
                shape.contains(&Point::new(pos.x as f64, pos.y as f64))
            )
            .count()
    });

    dbg!(inner_count);
}

fn time<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R
{
    let start = Instant::now();
    let result = f();

    eprintln!("{label}: {}ms", start.elapsed().as_micros() as f64 / 1000.0);

    result
}
