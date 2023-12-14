use aoc2023::day13::Patterns;

const INPUT: &str = include_str!("../inputs/day13-mirrors.txt");

fn main() {
    let patterns = Patterns::parse(INPUT).unwrap();

    dbg!(patterns.patterns.len());

    let sum: usize = patterns.patterns.iter()
        .enumerate()
        .map(|(i, pat)| {
            dbg!(i, pat.find_mirror_axis(|_, _| true));
            match pat.find_mirror_axis(|_, _| true) {
                (Some(x), None) => x + 1,
                (None, Some(y)) => (y + 1) * 100,
                _ => panic!()
            }
        })
        .sum();

    dbg!(sum);
}