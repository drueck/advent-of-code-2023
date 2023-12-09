use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    kind: Kind,
    cards: [u8; 5],
    pub bid: usize,
}

impl Hand {
    pub fn parse_cards(s: &str) -> [u8; 5] {
        let cards: [u8; 5] = s
            .as_bytes()
            .iter()
            .map(|&char_byte| match char_byte {
                n if n.is_ascii_digit() => n - b'0',
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .expect("hand is exactly five cards");

        cards
    }

    pub fn calculate_kind(cards: &[u8; 5]) -> Kind {
        let mut card_counts: HashMap<u8, u8> = HashMap::new();

        for card in cards {
            *card_counts.entry(*card).or_insert(0) += 1;
        }

        match &card_counts.len() {
            1 => Kind::FiveOfAKind,
            2 => match &card_counts.values().max().unwrap() {
                4 => Kind::FourOfAKind,
                3 => Kind::FullHouse,
                _ => unreachable!(),
            },
            3 => match &card_counts.values().any(|&count| count == 3) {
                true => Kind::ThreeOfAKind,
                false => Kind::TwoPair,
            },
            4 => Kind::OnePair,
            5 => Kind::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let cards: [u8; 5] = Hand::parse_cards(&parts[0]);
        let kind = Hand::calculate_kind(&cards);
        let bid = parts[1].parse().unwrap();

        Ok(Hand { kind, cards, bid })
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Hand, Kind};

    #[test]
    fn test_parse_cards() {
        assert_eq!(Hand::parse_cards("AKQJT"), [14, 13, 12, 11, 10]);
        assert_eq!(Hand::parse_cards("3AJ92"), [3, 14, 11, 9, 2]);
    }

    #[test]
    fn test_calculate_kind() {
        let five_of_a_kind = Hand::parse_cards("11111");
        assert_eq!(Hand::calculate_kind(&five_of_a_kind), Kind::FiveOfAKind);

        let four_of_a_kind = Hand::parse_cards("11211");
        assert_eq!(Hand::calculate_kind(&four_of_a_kind), Kind::FourOfAKind);

        let full_house = Hand::parse_cards("11221");
        assert_eq!(Hand::calculate_kind(&full_house), Kind::FullHouse);

        let three_of_a_kind = Hand::parse_cards("11123");
        assert_eq!(Hand::calculate_kind(&three_of_a_kind), Kind::ThreeOfAKind);

        let two_pair = Hand::parse_cards("11223");
        assert_eq!(Hand::calculate_kind(&two_pair), Kind::TwoPair);

        let one_pair = Hand::parse_cards("11234");
        assert_eq!(Hand::calculate_kind(&one_pair), Kind::OnePair);

        let high_card = Hand::parse_cards("12345");
        assert_eq!(Hand::calculate_kind(&high_card), Kind::HighCard);
    }

    #[test]
    fn test_parse_hand() {
        let text = "32T3K 765";
        let hand: Hand = text.parse().unwrap();

        let expected_hand = Hand {
            kind: Kind::OnePair,
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        };

        assert_eq!(hand, expected_hand);
    }

    #[test]
    fn test_ordering() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut hands: Vec<Hand> = input
            .trim()
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();

        hands.sort_unstable();

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 220);
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[3].bid, 684);
        assert_eq!(hands[4].bid, 483);
    }
}
