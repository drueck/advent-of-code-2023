// Advent of Code 2023: Day 1
// https://adventofcode.com/2023/day/1
// Usage: `cargo run <input-file> [--only-digits]

use std::collections::HashMap;
use std::env;
use std::fs;

const MAX_WORD_LENGTH: usize = 3;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");

    let only_digits = match env::args().nth(2) {
        Some(val) if val == "--only-digits" => true,
        _ => false,
    };

    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum_of_calibration_values: usize = input
        .trim()
        .split("\n")
        .map(|line| {
            let line_length = line.len();
            let line_bytes = line.as_bytes();
            let mut digits = vec![];

            for i in 0..line_length {
                match line_bytes[i] {
                    digit if digit >= b'0' && digit <= b'9' => digits.push((digit - b'0') as usize),
                    _ => {
                        if only_digits || i + MAX_WORD_LENGTH > line_length {
                            continue;
                        }

                        for (&word, digit) in &words {
                            if i + word.len() > line_length {
                                continue;
                            }

                            if line[i..][..word.len()] == *word {
                                digits.push(*digit)
                            }
                        }
                    }
                }
            }

            digits.first().expect("there is a first digit") * 10
                + digits.last().expect("there is a second digit")
        })
        .sum();

    println!(
        "The sum of the calibration values is: {}",
        sum_of_calibration_values
    );
}
