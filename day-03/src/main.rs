// Advent of Code 2023: Day 3
// https://adventofcode.com/2023/day/3
// Usage: `cargo run <input-file>

use day_03::schematic::Schematic;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let schematic = Schematic::new(&input);

    let sum_of_part_numbers: usize = schematic.part_numbers().iter().sum();

    println!("The sum of the part numbers is {}", sum_of_part_numbers);
}
