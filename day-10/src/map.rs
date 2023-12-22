use std::collections::{BinaryHeap, HashMap};
use Direction::*;

#[derive(Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    location: usize,
}

impl State {
    pub fn new(location: usize, steps: usize) -> Self {
        Self { location, steps }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(other.location.cmp(&self.location))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

pub struct Map {
    map: Box<[u8]>,
    start: usize,
    columns: usize,
    rows: usize,
    connections: HashMap<usize, [Option<usize>; 4]>,
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let columns = input.trim().find('\n').unwrap();
        let input_without_newlines = input.trim().replace('\n', "");
        let map = input_without_newlines.as_bytes();
        let rows = map.len() / columns;
        let start = map.iter().position(|&b| b == b'S').unwrap();

        Self {
            map: map.into(),
            columns,
            rows,
            start,
            connections: HashMap::with_capacity(map.len()),
        }
    }

    fn column(&self, i: usize) -> usize {
        i % self.columns
    }

    fn row(&self, i: usize) -> usize {
        i / self.columns
    }

    fn directions(pipe: u8) -> [Option<Direction>; 4] {
        match pipe {
            b'|' => [Some(Up), Some(Down), None, None],
            b'-' => [Some(Left), Some(Right), None, None],
            b'L' => [Some(Up), Some(Right), None, None],
            b'J' => [Some(Up), Some(Left), None, None],
            b'7' => [Some(Down), Some(Left), None, None],
            b'F' => [Some(Down), Some(Right), None, None],
            b'S' => [Some(Up), Some(Right), Some(Down), Some(Left)],
            b'.' => [None, None, None, None],
            _ => unreachable!(),
        }
    }

    pub fn compute_connetions(&mut self) {
        // insert the connections to which the pipes are pointing
        for i in 0..self.map.len() {
            self.connections.insert(i, self.connections_from(i));
        }

        // remove connections that aren't reciprocated by the receiving location
        for i in 0..self.map.len() {
            let mut connections = self.connections[&i];
            for connection in &mut connections {
                if let Some(connected_index) = connection {
                    if !self.connections[connected_index]
                        .as_slice()
                        .contains(&Some(i))
                    {
                        *connection = None;
                    }
                }
            }

            *self.connections.get_mut(&i).unwrap() = connections;
        }
    }

    fn connections_from(&self, i: usize) -> [Option<usize>; 4] {
        let mut connections: [Option<usize>; 4] = [None; 4];

        let can_go_up = i >= self.columns;
        let can_go_right = self.column(i) < self.columns - 1;
        let can_go_down = self.row(i) < self.rows - 1;
        let can_go_left = self.column(i) > 0;

        for (n, direction) in Self::directions(self.map[i]).into_iter().enumerate() {
            match direction {
                Some(Up) => connections[n] = can_go_up.then(|| i - self.columns),
                Some(Right) => connections[n] = can_go_right.then(|| i + 1),
                Some(Down) => connections[n] = can_go_down.then(|| i + self.columns),
                Some(Left) => connections[n] = can_go_left.then(|| i - 1),
                None => {}
            }
        }

        connections
    }

    pub fn steps_to_farthest_location(&self) -> usize {
        let mut min_steps_to_location: HashMap<usize, usize> = HashMap::new();
        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        queue.push(State::new(self.start, 0));
        min_steps_to_location.insert(self.start, 0);

        while let Some(state) = queue.pop() {
            for connection in self.connections[&state.location].iter().flatten() {
                let steps = state.steps + 1;
                let min_steps = min_steps_to_location
                    .entry(*connection)
                    .or_insert(usize::MAX);
                if steps < *min_steps {
                    *min_steps = steps;
                    queue.push(State::new(*connection, steps));
                }
            }
        }

        *min_steps_to_location.values().max().unwrap()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_state_ordering() {
        let first = State::new(2, 1);
        let second = State::new(1, 2);
        let third = State::new(2, 2);

        assert!(first > second);
        assert!(second > third);
    }
}
