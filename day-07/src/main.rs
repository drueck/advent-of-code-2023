// Advent of Code 2023: Day 7
// https://adventofcode.com/2023/day/7
// Usage: `cargo run <input-file>

use day_07::hand::Hand;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    println!(
        "The total winnings if J is a Jack are {}",
        calculate_winnings(&input)
    );
    println!(
        "The total winnings if J is a Joker are {}",
        calculate_winnings(&input.replace('J', "1"))
    );
}

fn calculate_winnings(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .trim()
        .split('\n')
        .map(|hand| hand.parse().unwrap())
        .collect();

    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .fold(0, |winnings, (i, hand)| winnings + (i + 1) * hand.bid)
}
