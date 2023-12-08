// Advent of Code 2023: Day 6
// https://adventofcode.com/2023/day/6
// Usage: `cargo run <input-file>

use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
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
        .map(|i| {
            let race_time = times[i];
            let record_distance = distances[i];

            (1..(race_time - 1))
                .map(|hold_time| distance(race_time, hold_time))
                .filter(|distance| distance > &record_distance)
                .count()
        })
        .product();

    println!(
        "The product of the ways we could beat each race is: {}",
        product_of_better_outcomes
    );
}

pub fn distance(race_time: usize, hold_time: usize) -> usize {
    hold_time * (race_time - hold_time)
}

#[cfg(test)]
pub mod tests {
    use crate::distance;

    #[test]
    fn test_distance() {
        assert_eq!(distance(7, 1), 6);
        assert_eq!(distance(7, 5), 10);
    }
}
