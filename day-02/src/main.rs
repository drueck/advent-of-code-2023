// Advent of Code 2023: Day 2
// https://adventofcode.com/2023/day/2
// Usage: `cargo run <input-file>

use std::cmp::max;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl Game {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> Option<usize> {
        if self.max_red <= red && self.max_green <= green && self.max_blue <= blue {
            return Some(self.id);
        }
        None
    }

    fn power(&self) -> usize {
        self.max_red * self.max_green * self.max_blue
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut major_parts = s.split(':');
        let id: usize = major_parts
            .next()
            .expect("game section exists")
            .split(' ')
            .last()
            .expect("game id exists")
            .parse()
            .expect("game id is a positive integer");

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for mut cube_info in major_parts
            .next()
            .expect("game details are in expected format")
            .trim()
            .split([',', ';'])
            .map(|game_part| game_part.trim().split(' '))
        {
            let number: usize = cube_info
                .next()
                .expect("has a number")
                .parse()
                .expect("number is a positive integer");

            let color = cube_info.next().expect("has a color");

            match color {
                "red" => {
                    max_red = max(max_red, number);
                }
                "green" => {
                    max_green = max(max_green, number);
                }
                "blue" => {
                    max_blue = max(max_blue, number);
                }
                _ => {
                    unreachable!()
                }
            }
        }

        Ok(Game {
            id,
            max_red,
            max_green,
            max_blue,
        })
    }
}

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");

    let red = 12;
    let green = 13;
    let blue = 14;

    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let games: Vec<Game> = input
        .trim()
        .split("\n")
        .map(|line| line.parse().expect("valid game string format"))
        .collect();

    let sum_of_possible_games: usize = games
        .iter()
        .filter_map(|game| game.is_possible(red, green, blue))
        .sum();

    let sum_of_power_of_sets: usize = games.iter().map(Game::power).sum();

    println!(
        "The sum of the possible games for rgb({}, {}, {}) is: {}",
        red, green, blue, sum_of_possible_games
    );

    println!(
        "The sum of the powers of each set is: {}",
        sum_of_power_of_sets
    );
}
