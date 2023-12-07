use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
pub struct Map {
    pub from: String,
    pub to: String,
    pub source_ranges: Vec<RangeInclusive<usize>>,
    pub destination_ranges: Vec<RangeInclusive<usize>>,
}

impl Map {
    pub fn get(&self, source: usize) -> usize {
        for range_i in 0..self.source_ranges.len() {
            let source_range = &self.source_ranges[range_i];
            let destination_range = &self.destination_ranges[range_i];

            if source_range.contains(&source) {
                return destination_range.start() + (source - source_range.start());
            }
        }
        source
    }
}

#[derive(Debug)]
pub struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Map, Self::Err> {
        let name_re = Regex::new(r"(?<from>.+)-to-(?<to>.+) map:").map_err(|_| ParseMapError)?;

        let mut lines = s.split('\n');
        let name_parts = name_re.captures(lines.next().unwrap()).unwrap();
        let from = String::from(&name_parts["from"]);
        let to = String::from(&name_parts["to"]);

        let mut source_ranges = vec![];
        let mut destination_ranges = vec![];

        for line in lines {
            let numbers: Vec<_> = line
                .trim()
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            source_ranges.push(numbers[1]..=(numbers[1] + numbers[2] - 1));
            destination_ranges.push(numbers[0]..=(numbers[0] + numbers[2] - 1));
        }

        Ok(Map {
            from,
            to,
            source_ranges,
            destination_ranges,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::map::Map;

    #[test]
    fn test_parse() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map: Map = text.parse().unwrap();

        assert_eq!(map.from, "seed");
        assert_eq!(map.to, "soil");
        assert_eq!(map.source_ranges, vec![98..=99, 50..=97]);
        assert_eq!(map.destination_ranges, vec![50..=51, 52..=99]);
    }
}
