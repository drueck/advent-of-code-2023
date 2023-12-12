// Advent of Code 2023: Day 8
// https://adventofcode.com/2023/day/8
// Usage: `cargo run <input-file>

use primes::{PrimeSet, Sieve};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

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

    let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();

    let map: HashMap<&str, [&str; 2]> = parts
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let (_, map_parts): (&str, [&str; 3]) = re.captures(&line).unwrap().extract();
            (map_parts[0], [map_parts[1], map_parts[2]])
        })
        .collect();

    part_one(&map, &directions);
    part_two(&map, &directions);
}

fn part_one(map: &HashMap<&str, [&str; 2]>, directions: &Vec<usize>) {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let mut location = START;
    let mut steps = 0;

    for &direction in directions.iter().cycle() {
        location = map.get(&location).unwrap()[direction];
        steps += 1;
        if location == END {
            break;
        }
    }

    println!("Navigated from {} to {} in {} steps", START, END, steps);
}

fn part_two(map: &HashMap<&str, [&str; 2]>, directions: &Vec<usize>) {
    let locations: Vec<&str> = map
        .keys()
        .filter_map(|&location| location.ends_with('A').then_some(location))
        .collect();

    let mut steps_to_destinations: Vec<u64> = vec![];

    for &start in locations.iter() {
        let mut location = start;
        let mut steps: u64 = 0;

        for &direction in directions.iter().cycle() {
            steps += 1;
            location = map.get(location).unwrap()[direction];
            if location.ends_with('Z') {
                steps_to_destinations.push(steps);
                break;
            }
        }
    }

    if let Some(lcm) = lowest_common_multiple(&mut steps_to_destinations) {
        println!("All the ghosts reached their destinations in {} steps", lcm);
    }
}

fn lowest_common_multiple(numbers: &mut Vec<u64>) -> Option<u64> {
    let lowest_number: u64 = *numbers.iter().min().unwrap();
    let mut prime_factors: HashSet<u64> = HashSet::new();
    let mut primes = Sieve::new();

    for prime in primes.iter() {
        for i in 0..numbers.len() {
            if numbers[i] % prime == 0 {
                prime_factors.insert(prime);
                numbers[i] = numbers[i] / prime;
            }
        }
        if prime > lowest_number || numbers.iter().all(|&n| n == 1) {
            break;
        }
    }

    return Some(prime_factors.iter().product());
}
