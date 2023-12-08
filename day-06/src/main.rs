// Advent of Code 2023: Day 6
// https://adventofcode.com/2023/day/6
// Usage: `cargo run <input-file>

use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut lines = input.trim().split('\n');

    let numbers_re = Regex::new(r"(\d+)").unwrap();

    let times: Vec<_> = numbers_re
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();

    let distances: Vec<_> = numbers_re
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();

    let num_races = times.len();

    let product_of_better_outcomes: usize = (0..num_races)
        .map(|i| ways_to_win(times[i], distances[i]))
        .product();

    println!(
        "The product of the ways we could beat each race is: {}",
        product_of_better_outcomes
    );
}

fn part_two(input: &str) {
    let numbers_re = Regex::new(r"(\d+)").unwrap();
    let collapsed_input = input.replace(' ', "");
    let mut numbers = numbers_re.find_iter(&collapsed_input);

    let race_time = numbers
        .next()
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .unwrap();

    let previous_record = numbers
        .next()
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .unwrap();

    println!(
        "The number of ways to beat the previous record are: {}",
        ways_to_win(race_time, previous_record)
    );
}

fn distance(race_time: usize, hold_time: usize) -> usize {
    hold_time * (race_time - hold_time)
}

fn ways_to_win(race_time: usize, record: usize) -> usize {
    let common = ((race_time.pow(2) - 4 * record) as f64).sqrt();
    let mut low = ((race_time as f64 - common) / 2.0).floor() as usize;
    let mut high = ((race_time as f64 + common) / 2.0).floor() as usize + 1;

    while distance(race_time, low) <= record {
        low += 1;
    }

    while distance(race_time, high) <= record {
        high -= 1;
    }

    high - low + 1
}

#[cfg(test)]
pub mod tests {
    use crate::{distance, ways_to_win};

    #[test]
    fn test_distance() {
        assert_eq!(distance(7, 2), 10);
        assert_eq!(distance(7, 6), 6);
    }

    #[test]
    fn test_ways_to_win() {
        assert_eq!(ways_to_win(7, 9), 4);
        assert_eq!(ways_to_win(15, 40), 8);
        assert_eq!(ways_to_win(30, 200), 9);
        assert_eq!(ways_to_win(71530, 940200), 71503);
    }
}
