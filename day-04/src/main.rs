// Advent of Code 2023: Day 4
// https://adventofcode.com/2023/day/4
// Usage: `cargo run <input-file>

use day_04::card::Card;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let cards: Vec<Card> = input
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut card_counts: HashMap<usize, usize> =
        HashMap::from_iter((0..cards.len()).map(|i| (i, 1)));

    for (i, card) in cards.iter().enumerate() {
        for j in (i + 1)..=(i + card.matches()) {
            *card_counts.entry(j).or_insert(1) += card_counts[&i];
        }
    }

    let total: usize = cards.iter().map(|card| card.score()).sum();
    let card_count: usize = card_counts.into_values().sum();

    println!("The total score of all the cards was {}", total);
    println!("The total cards accumulated was {}", card_count);
}
