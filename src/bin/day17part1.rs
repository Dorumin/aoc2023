use aoc2023::day17::{Crucible, Direction};

const INPUT: &str = include_str!("../inputs/day17-crucible.txt");

fn main() {
    let crucible = Crucible::parse(INPUT).unwrap();
    let start_position = crucible.get_position(0, 0, 0, Direction::Right);
    let end_position = crucible.get_position(crucible.width() - 1, crucible.height() - 1, 0, Direction::Right);

    let path = start_position.pathfind_to(&end_position, &crucible, 3);

    let total_heat: usize = path.iter().map(|p| p.heat as usize).sum();

    // dbg!(&path);

    crucible.print_path(&path);

    dbg!(path.len());
    dbg!(total_heat);
}