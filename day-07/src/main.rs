// Advent of Code 2023: Day 7
// https://adventofcode.com/2023/day/7
// Usage: `cargo run <input-file>

use std::env;
use std::fs;

use day_07::hand::Hand;
use day_07::joker_hand::JokerHand;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut hands: Vec<Hand> = input
        .trim()
        .split('\n')
        .map(|hand| hand.parse().unwrap())
        .collect();

    hands.sort_unstable();

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |winnings, (i, hand)| winnings + (i + 1) * hand.bid);

    println!("The total winnings if J is a Jack is {}", winnings);
}

fn part_two(input: &str) {
    let mut hands: Vec<JokerHand> = input
        .trim()
        .split('\n')
        .map(|hand| hand.parse().unwrap())
        .collect();

    hands.sort_unstable();

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |winnings, (i, hand)| winnings + (i + 1) * hand.bid);

    println!("The total winnings if J is a Joker is {}", winnings);
}
