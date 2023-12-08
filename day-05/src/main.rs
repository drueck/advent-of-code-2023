// Advent of Code 2023: Day 5
// https://adventofcode.com/2023/day/5
// Usage: `cargo run <input-file>

// credit for the part two algorithm goes to reddit user zuleyorker.

use day_05::map::Map;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
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

    println!(
        "The lowest location with part one rules is: {}",
        lowest_location
    );
}

fn part_two(input: &str) {
    let mut sections = input.trim().split("\n\n");

    let seeds_re = Regex::new(r"(\d+)").unwrap();

    let seed_ranges: Vec<RangeInclusive<usize>> = seeds_re
        .find_iter(sections.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect::<Vec<usize>>()[..]
        .chunks(2)
        .map(|pair| pair[0]..=(pair[0] + pair[1] - 1))
        .collect();

    let maps: Vec<Map> = sections.map(|s| s.parse().unwrap()).collect();

    // try to find a smaller number of seeds to check by working backward through the maps
    let mut seeds_to_check = maps
        .iter()
        .rev()
        // start with the endpoints of the whole usize range
        .fold(HashSet::from([0, usize::MAX]), |set, map| {
            // for each map working backward
            // translate all current values by reversing the mapping
            let mut updated: HashSet<_> = set.iter().map(|&value| map.get_reverse(value)).collect();
            // then add the endpoints from the map's source range
            updated.extend(map.source_range_endpoints());
            updated
        });

    // limit the endpoints to check to values in the known seed ranges
    seeds_to_check.retain(|value| seed_ranges.iter().any(|range| range.contains(value)));

    // add the endpoints of the seed ranges themselves
    seeds_to_check.extend(seed_ranges.iter().flat_map(|r| vec![r.start(), r.end()]));

    // now that we have a limited number of seeds to check, translate to locations and get the min
    let locations = maps.iter().fold(seeds_to_check, |results, map| {
        results.iter().map(|&source| map.get(source)).collect()
    });

    let lowest_location = locations.iter().min().unwrap();

    println!(
        "The lowest location with part two rules is: {}",
        lowest_location
    );
}
