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
    map.compute_connections();
    map.find_loop_path();

    println!(
        "The farthest location is {} steps away",
        map.steps_to_farthest_part_of_loop()
    );

    println!(
        "The number of tiles enclosed by the loop are {}",
        map.tiles_inside_loop()
    )
}
