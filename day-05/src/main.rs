// Advent of Code 2023: Day 5
// https://adventofcode.com/2023/day/5
// Usage: `cargo run <input-file>

use day_05::map::Map;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut sections = input.trim().split("\n\n");

    let seeds_re = Regex::new(r"(\d+)").unwrap();

    let seeds: Vec<usize> = seeds_re
        .find_iter(sections.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let maps: Vec<Map> = sections.map(|s| s.parse().unwrap()).collect();

    let locations = maps.iter().fold(seeds, |seeds, map| {
        seeds.iter().map(|&seed| map.get(seed)).collect()
    });

    let lowest_location = locations.iter().min().unwrap();

    println!("The lowest location was {}", lowest_location);
}
