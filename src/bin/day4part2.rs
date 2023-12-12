use aoc2023::day4::Scratchcard;

const INPUT: &str = include_str!("../inputs/day4-scratchcards.txt");

fn main() {
    let mut cards: Vec<_> = INPUT.lines()
        .flat_map(Scratchcard::parse_line)
        .collect();

    let max_index = cards.len();

    for i in 0.. {
        if i >= cards.len() {
            break;
        }

        let card: &Scratchcard = &cards[i];
        let game_id = card.game_id;

        for n in 0..card.gotten_winning_numbers() {
            let next_index = game_id + n;

            // No cards past the end of the original cards
            if next_index >= max_index {
                break;
            }

            let next_card = &cards[next_index];
            assert_eq!(next_card.game_id, next_index + 1);

            cards.push(next_card.clone());
        }
    }

    dbg!(cards.len());
}