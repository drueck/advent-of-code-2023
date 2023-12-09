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
pub struct JokerHand {
    kind: Kind,
    cards: [u8; 5],
    pub bid: usize,
}

impl JokerHand {
    pub fn parse_cards(s: &str) -> [u8; 5] {
        let cards: [u8; 5] = s
            .as_bytes()
            .iter()
            .map(|&char_byte| match char_byte {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'T' => 10,
                b'J' => 1,
                n if n.is_ascii_digit() => n - b'0',
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

        if let Some(num_jokers) = card_counts.remove(&1) {
            match num_jokers {
                5 => {
                    card_counts.insert(14, 5);
                }
                n => {
                    let most_prevalent_card = card_counts
                        .iter()
                        .max_by(|a, b| a.1.cmp(&b.1))
                        .map(|(card, _count)| card)
                        .unwrap();

                    *card_counts.entry(*most_prevalent_card).or_insert(0) += n;
                }
            }
        };

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
pub struct ParseJokerHandError;

impl FromStr for JokerHand {
    type Err = ParseJokerHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let cards: [u8; 5] = JokerHand::parse_cards(&parts[0]);
        let kind = JokerHand::calculate_kind(&cards);
        let bid = parts[1].parse().unwrap();

        Ok(JokerHand { kind, cards, bid })
    }
}

#[cfg(test)]
pub mod tests {
    use super::{JokerHand, Kind};

    #[test]
    fn test_parse_cards() {
        assert_eq!(JokerHand::parse_cards("AKQJT"), [14, 13, 12, 1, 10]);
        assert_eq!(JokerHand::parse_cards("3AJ92"), [3, 14, 1, 9, 2]);
    }

    #[test]
    fn test_calculate_kind() {
        let five_of_a_kind = JokerHand::parse_cards("22221");
        assert_eq!(
            JokerHand::calculate_kind(&five_of_a_kind),
            Kind::FiveOfAKind
        );

        let four_of_a_kind = JokerHand::parse_cards("77271");
        assert_eq!(
            JokerHand::calculate_kind(&four_of_a_kind),
            Kind::FourOfAKind
        );

        let full_house = JokerHand::parse_cards("77221");
        assert_eq!(JokerHand::calculate_kind(&full_house), Kind::FullHouse);

        let three_of_a_kind = JokerHand::parse_cards("17723");
        assert_eq!(
            JokerHand::calculate_kind(&three_of_a_kind),
            Kind::ThreeOfAKind
        );

        let two_pair = JokerHand::parse_cards("77223");
        assert_eq!(JokerHand::calculate_kind(&two_pair), Kind::TwoPair);

        let one_pair = JokerHand::parse_cards("71234");
        assert_eq!(JokerHand::calculate_kind(&one_pair), Kind::OnePair);

        let high_card = JokerHand::parse_cards("72345");
        assert_eq!(JokerHand::calculate_kind(&high_card), Kind::HighCard);
    }

    #[test]
    fn test_parse_hand() {
        let text = "32T3K 765";
        let hand: JokerHand = text.parse().unwrap();

        let expected_hand = JokerHand {
            kind: Kind::OnePair,
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        };

        assert_eq!(hand, expected_hand);
    }

    #[test]
    fn test_ordering() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut hands: Vec<JokerHand> = input
            .trim()
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();

        hands.sort_unstable();

        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].bid, 28);
        assert_eq!(hands[2].bid, 684);
        assert_eq!(hands[3].bid, 483);
        assert_eq!(hands[4].bid, 220);
    }
}
