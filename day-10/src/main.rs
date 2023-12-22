// Advent of Code 2023: Day 10
// https://adventofcode.com/2023/day/10
// Usage: `cargo run <input-file>

use day_10::map::Map;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut map = Map::new(&input);
    map.compute_connetions();

    println!(
        "The farthest location is {} steps away",
        map.steps_to_farthest_location()
    );
}
