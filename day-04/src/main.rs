// Advent of Code 2023: Day 4
// https://adventofcode.com/2023/day/4
// Usage: `cargo run <input-file>

use regex::Regex;
use std::env;
use std::fs;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    winners: HashSet<usize>,
    ours: HashSet<usize>,
}

impl Card {
    pub fn score(&self) -> usize {
        match self.winners.intersection(&self.ours).count() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let re = Regex::new(r"(\d+)").unwrap();

    let cards: Vec<_> = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line.split('|');

            let mut before_pipe_numbers = re
                .find_iter(parts.next().unwrap())
                .map(|m| m.as_str().parse::<usize>().unwrap());

            let _number = before_pipe_numbers.next().unwrap();
            let winners = before_pipe_numbers.collect();

            let ours = re
                .find_iter(parts.next().unwrap())
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .collect();

            Card { winners, ours }
        })
        .collect();

    let total: usize = cards.iter().map(|card| card.score()).sum();

    println!("The total score of all the cards was {}", total);
}

#[cfg(test)]
pub mod tests {
    use crate::Card;
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
