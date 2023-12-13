// Advent of Code 2023: Day 9
// https://adventofcode.com/2023/day/9
// Usage: `cargo run <input-file>

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let sequences: Vec<Vec<isize>> = input
        .trim()
        .split('\n')
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect();

    let nexts: Vec<isize> = sequences.iter().map(next_in_sequence).collect();
    let sum_of_nexts: isize = nexts.iter().sum();

    let previouses: Vec<isize> = sequences.iter().map(previous_in_sequence).collect();
    let sum_of_previouses: isize = previouses.iter().sum();

    println!(
        "The sum of the next items in the sequences is {}",
        sum_of_nexts
    );
    println!(
        "The sum of the previous items in the sequences is {}",
        sum_of_previouses
    );
}

fn next_in_sequence(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|&val| val == 0) {
        return 0;
    }
    sequence.last().unwrap() + next_in_sequence(&diffs(sequence))
}

fn previous_in_sequence(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|&val| val == 0) {
        return 0;
    }
    sequence.first().unwrap() - previous_in_sequence(&diffs(sequence))
}

fn diffs(sequence: &Vec<isize>) -> Vec<isize> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}
