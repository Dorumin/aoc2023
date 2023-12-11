use aoc2023::day10::Map;

const INPUT: &str = include_str!("../inputs/day10-mario.txt");

fn main() {
    let map = Map::parse(INPUT).unwrap();

    println!("{width} {height}", width = map.width, height = map.height);
    println!("{map}");

    let path = map.follow(map.start()).unwrap();
    let path_length = path.len();
    let halfway_point = path_length / 2;

    println!("{}", map.display_path(&path));
    dbg!(halfway_point);
}