// Advent of Code 2023: Day 8
// https://adventofcode.com/2023/day/8
// Usage: `cargo run <input-file>

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const START: &str = "AAA";
const END: &str = "ZZZ";

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut parts = input.trim().split("\n\n");

    let directions: Vec<_> = parts
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'L' => 0,
            b'R' => 1,
            _ => unreachable!(),
        })
        .collect();

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let map: HashMap<&str, [&str; 2]> = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let (_, map_parts): (&str, [&str; 3]) = re.captures(&line).unwrap().extract();
            (map_parts[0], [map_parts[1], map_parts[2]])
        })
        .collect();

    let mut location = START;
    let mut steps = 0;

    for &direction in directions.iter().cycle() {
        location = map.get(&location).unwrap()[direction];
        steps += 1;
        if location == END {
            break;
        }
    }

    println!("Reached the end in {} steps", steps);
}
