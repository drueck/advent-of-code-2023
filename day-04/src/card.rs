use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    winners: HashSet<usize>,
    ours: HashSet<usize>,
}

impl Card {
    pub fn score(&self) -> usize {
        match self.matches() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }

    pub fn matches(&self) -> usize {
        self.winners.intersection(&self.ours).count()
    }
}

#[derive(Debug)]
pub struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)").unwrap();

        let mut parts = s.split('|');

        let before_pipe_numbers = re
            .find_iter(parts.next().unwrap())
            .map(|m| m.as_str().parse::<usize>().unwrap());

        let winners = before_pipe_numbers.skip(1).collect();

        let ours = re
            .find_iter(parts.next().unwrap())
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect();

        Ok(Self { winners, ours })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::card::Card;
    use std::collections::HashSet;

    #[test]
    fn test_score() {
        let zero_matches = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([]),
        };

        let one_match = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([1]),
        };

        let two_matches = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([1, 2]),
        };

        let three_matches = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([1, 2, 3]),
        };

        let four_matches = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([1, 2, 3, 4]),
        };

        let five_matches = Card {
            winners: HashSet::from([1, 2, 3, 4, 5]),
            ours: HashSet::from([1, 2, 3, 4, 5]),
        };

        assert_eq!(zero_matches.score(), 0);
        assert_eq!(one_match.score(), 1);
        assert_eq!(two_matches.score(), 2);
        assert_eq!(three_matches.score(), 4);
        assert_eq!(four_matches.score(), 8);
        assert_eq!(five_matches.score(), 16);
    }
}
