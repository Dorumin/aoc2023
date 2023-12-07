use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ruleset {
    Simple,
    Jokers
}

#[derive(Debug)]
pub struct Game {
    pub plays: Vec<Play>,
    pub ruleset: Ruleset
}

impl Game {
    pub fn parse(input: &str, ruleset: Ruleset) -> Game {
        Game {
            plays: input.lines().map(|line| Play::parse_line(line, ruleset).unwrap()).collect(),
            ruleset
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Play {
    pub cards: [Card; 5],
    pub bid: u64,
    ruleset: Ruleset
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
    pub fn parse_line(line: &str, ruleset: Ruleset) -> Option<Play> {
        let mut split = line.split_ascii_whitespace();
        let cards: Vec<_> = split.next()?.chars().map(|letter| Card::from_char(letter, ruleset).unwrap()).collect();
        let bid = split.next()?.parse().ok()?;

        Some(Play {
            cards: cards.try_into().ok()?,
            bid,
            ruleset
        })
    }

    pub fn hand_type(&self) -> HandType {
        match self.ruleset {
            Ruleset::Simple => self.hand_type_simple(),
            Ruleset::Jokers => self.hand_type_jokers(),
        }
    }

    pub fn hand_type_jokers(&self) -> HandType {
        let mut counts = HashMap::new();

        for card in self.cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        if let Some(jojokers) = counts.remove(&Card::Joker) {
            let max_card = counts.iter()
                .max_by_key(|(_key, count)| **count)
                .map(|(key, _)| *key )
                .unwrap_or_else(|| &Card::A); // God forbid, if there's five jokers, let's just make them all acers

            *counts.entry(max_card).or_insert(0) += jojokers;
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

    pub fn hand_type_simple(&self) -> HandType {
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
    Two,
    Joker
}

impl Card {
    fn from_char(card: char, ruleset: Ruleset) -> Option<Card> {
        let card = match card {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' if matches!(ruleset, Ruleset::Jokers) => Card::Joker,
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