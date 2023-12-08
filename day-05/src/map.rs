use regex::Regex;
use std::collections::HashSet;
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

    pub fn get_reverse(&self, destination: usize) -> usize {
        for range_i in 0..self.destination_ranges.len() {
            let source_range = &self.source_ranges[range_i];
            let destination_range = &self.destination_ranges[range_i];

            if destination_range.contains(&destination) {
                return source_range.start() + (destination - destination_range.start());
            }
        }
        destination
    }

    /// return a set containing the start and end of each range,
    /// as well as the values lower and higher than each range
    /// which form the endpoints of the bounds outside the specified ranges
    /// we exclude 0 and usize::MAX because those are assumed
    pub fn source_range_endpoints(&self) -> HashSet<usize> {
        self.source_ranges
            .iter()
            .fold(HashSet::new(), |mut endpoints, range| {
                let start = *range.start();
                let end = *range.end();

                if start > 1 {
                    endpoints.insert(start - 1);
                }
                endpoints.insert(start);
                endpoints.insert(end);
                endpoints.insert(end + 1);
                endpoints
            })
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
    use std::collections::HashSet;

    #[test]
    fn test_parse() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map: Map = text.parse().unwrap();

        assert_eq!(map.from, "seed");
        assert_eq!(map.to, "soil");
        assert_eq!(map.source_ranges, vec![98..=99, 50..=97]);
        assert_eq!(map.destination_ranges, vec![50..=51, 52..=99]);
    }

    #[test]
    fn test_get() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map: Map = text.parse().unwrap();

        assert_eq!(map.get(98), 50);
        assert_eq!(map.get(99), 51);
        assert_eq!(map.get(50), 52);
        assert_eq!(map.get(55), 57);
        assert_eq!(map.get(97), 99);
    }

    #[test]
    fn test_get_reverse() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map: Map = text.parse().unwrap();

        assert_eq!(map.get_reverse(50), 98);
        assert_eq!(map.get_reverse(51), 99);
        assert_eq!(map.get_reverse(52), 50);
        assert_eq!(map.get_reverse(57), 55);
        assert_eq!(map.get_reverse(99), 97);
    }

    #[test]
    fn test_source_range_endpoints() {
        let text = "seed-to-soil map:\n50 98 2\n52 50 48";
        let map: Map = text.parse().unwrap();

        assert_eq!(
            map.source_range_endpoints(),
            HashSet::from([49, 50, 97, 98, 99, 100])
        )
    }
}
