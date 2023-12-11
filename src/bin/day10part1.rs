use aoc2023::day10::Map;

const INPUT: &str = include_str!("../inputs/day10-mario.txt");

fn main() {
    let oasis = Map::parse(INPUT).unwrap();

    println!("{width} {height}", width = oasis.width, height = oasis.height);
    println!("{oasis}");

    let path = oasis.follow(oasis.start()).unwrap();
    let path_length = path.len();
    let halfway_point = path_length / 2;

    println!("{}", oasis.display_path(&path));
    dbg!(halfway_point);
}