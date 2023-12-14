use aoc2023::day13::Patterns;

const INPUT: &str = include_str!("../inputs/day13-mirrors.txt");

fn main() {
    let patterns = Patterns::parse(INPUT).unwrap();

    let mut total: usize = 0;

    for pattern in patterns.patterns.iter() {
        let original_axis = pattern.find_mirror_axis(|_, _| true);
        let smudged = pattern.smudges().find(|s| !matches!(s.find_mirror_axis(|ax, i| {
            match ax {
                0 => original_axis.0 != Some(i),
                1 => original_axis.1 != Some(i),
                _ => panic!()
            }
        }), (None, None)));

        println!("smudge {:?}", smudged);

        if let Some(smudged) = smudged {
            let axis = smudged.find_mirror_axis(|ax, i| {
                match ax {
                    0 => original_axis.0 != Some(i),
                    1 => original_axis.1 != Some(i),
                    _ => panic!()
                }
            });
            dbg!(axis);

            match axis {
                (Some(x), None) => total += x + 1,
                (None, Some(y)) => total += (y + 1) * 100,
                _ => panic!()
            }
        }
    }

    dbg!(total);
    // dbg!(smudged_patterns.sum::<usize>());
}