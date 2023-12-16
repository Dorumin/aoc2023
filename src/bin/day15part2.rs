use aoc2023::day15::Sequence;

const INPUT: &str = include_str!("../inputs/day15-steps.txt");

fn main() {
    let seq = Sequence::parse(INPUT).unwrap();

    let slots = seq.fill_sluts();

    let mut result: usize = 0;

    for (slot_index, slot) in slots.iter().enumerate() {
        if !slot.is_empty() {
            println!("{slot_index} {:?}", slot.values());
        }

        for (lens_index, focal_length) in slot.values().enumerate() {
            result += (slot_index + 1) * (lens_index + 1) * (*focal_length as usize);
        }
    }

    dbg!(result);
}