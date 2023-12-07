use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    pub plays: Vec<Play>
}

impl Game {
    pub fn parse(input: &str) -> Game {
        Game {
            plays: input.lines().map(|line| Play::parse_line(line).unwrap()).collect()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Play {
    pub cards: [Card; 5],
    pub bid: u64
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compare hand type first
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(core::cmp::Ordering::Equal) => {},
            ord => return ord
        }

        // Fall back on card numbering
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        None
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Play {
    pub fn parse_line(line: &str) -> Option<Play> {
        let mut split = line.split_ascii_whitespace();
        let cards: Vec<_> = split.next()?.chars().map(|letter| Card::from_char(letter).unwrap()).collect();
        let bid = split.next()?.parse().ok()?;

        Some(Play {
            cards: cards.try_into().ok()?,
            bid
        })
    }

    pub fn hand_type(&self) -> HandType {
        let mut counts = HashMap::new();

        for card in self.cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        // Could be inlined to a length
        let mut values: Vec<_> = counts.values().collect();
        values.sort();

        match values[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card {
    fn from_char(card: char) -> Option<Card> {
        let card = match card {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return None
        };

        Some(card)
    }
}