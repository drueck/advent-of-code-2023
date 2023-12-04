pub struct Schematic {
    pub bytes: Box<[u8]>,
    pub columns: usize,
    pub rows: usize,
}

impl Schematic {
    pub fn new(source: &str) -> Self {
        let columns = source.trim().find('\n').expect("at least two rows");
        let string = source.trim().replace('\n', "");
        let rows = string.len() / columns;
        let bytes = string.as_bytes().into();

        Self {
            bytes,
            columns,
            rows,
        }
    }

    pub fn adjacent_indices(&self, i: usize) -> Vec<usize> {
        let mut adjacent_indices = vec![];

        let can_go_up = i >= self.columns;
        let can_go_right = i % self.columns < self.columns - 1;
        let can_go_down = i / self.columns < self.rows - 1;
        let can_go_left = i % self.columns > 0;

        if can_go_up {
            adjacent_indices.push(i - self.columns)
        }
        if can_go_up && can_go_right {
            adjacent_indices.push(i - self.columns + 1)
        }
        if can_go_right {
            adjacent_indices.push(i + 1)
        }
        if can_go_down && can_go_right {
            adjacent_indices.push(i + self.columns + 1)
        }
        if can_go_down {
            adjacent_indices.push(i + self.columns)
        }
        if can_go_down && can_go_left {
            adjacent_indices.push(i + self.columns - 1)
        }
        if can_go_left {
            adjacent_indices.push(i - 1)
        }
        if can_go_up && can_go_left {
            adjacent_indices.push(i - self.columns - 1)
        }

        adjacent_indices
    }

    pub fn adjacent_to_symbol(&self, i: usize) -> bool {
        self.adjacent_indices(i)
            .iter()
            .any(|&j| !self.bytes[j].is_ascii_digit() && self.bytes[j] != b'.')
    }

    pub fn part_numbers(&self) -> Vec<usize> {
        let mut part_numbers: Vec<usize> = vec![];
        let mut current_number_digits: Vec<usize> = vec![];
        let mut is_adjacent = false;

        for i in 0..self.bytes.len() {
            let byte = self.bytes[i];

            if i % self.columns == 0 || !byte.is_ascii_digit() {
                if !current_number_digits.is_empty() && is_adjacent {
                    let mut part_number = 0;
                    let mut place = 1;
                    for digit in current_number_digits.iter().rev() {
                        part_number += digit * place;
                        place *= 10;
                    }
                    part_numbers.push(part_number);
                }
                current_number_digits.clear();
                is_adjacent = false;
            }

            if byte.is_ascii_digit() {
                current_number_digits.push((byte - b'0') as usize);
                if is_adjacent {
                    continue;
                }
                if self.adjacent_to_symbol(i) {
                    is_adjacent = true;
                }
            }
        }

        part_numbers
    }
}

#[cfg(test)]
pub mod test {
    use crate::schematic::Schematic;
    use std::fs;

    #[test]
    fn test_new() {
        let input = fs::read_to_string("test-input.txt").expect("test input exists");

        let schematic = Schematic::new(&input);
        assert_eq!(schematic.columns, 10);
        assert_eq!(schematic.rows, 10);
    }

    #[test]
    fn test_adjacent_indices() {
        let input = fs::read_to_string("test-input.txt").expect("test input exists");
        let schematic = Schematic::new(&input);

        assert_eq!(schematic.adjacent_indices(0), vec![1, 11, 10]);
        assert_eq!(schematic.adjacent_indices(1), vec![2, 12, 11, 10, 0]);
        assert_eq!(
            schematic.adjacent_indices(11),
            vec![1, 2, 12, 22, 21, 20, 10, 0]
        );
        assert_eq!(schematic.adjacent_indices(9), vec![19, 18, 8]);
        assert_eq!(schematic.adjacent_indices(8), vec![9, 19, 18, 17, 7]);
        assert_eq!(schematic.adjacent_indices(90), vec![80, 81, 91]);
        assert_eq!(schematic.adjacent_indices(99), vec![89, 98, 88]);
    }

    #[test]
    fn test_adjacent_to_symbol() {
        let input = fs::read_to_string("test-input.txt").expect("test input exists");
        let schematic = Schematic::new(&input);

        assert_eq!(schematic.adjacent_to_symbol(0), false);
        assert_eq!(schematic.adjacent_to_symbol(1), false);
        assert_eq!(schematic.adjacent_to_symbol(2), true);
        assert_eq!(schematic.adjacent_to_symbol(93), true);
        assert_eq!(schematic.adjacent_to_symbol(99), false);
    }

    #[test]
    fn test_part_numbers() {
        let input = fs::read_to_string("test-input.txt").expect("test input exists");
        let schematic = Schematic::new(&input);
        let part_numbers = schematic.part_numbers();

        assert_eq!(part_numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
        assert_eq!(part_numbers.iter().sum::<usize>(), 4361);
    }
}
