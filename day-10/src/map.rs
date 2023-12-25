use std::collections::{BinaryHeap, HashMap};
use Direction::*;

#[derive(Debug, PartialEq, Eq)]
struct State {
    steps: usize,
    location: usize,
    path: Vec<usize>,
}

impl State {
    pub fn new(location: usize, steps: usize, path: Vec<usize>) -> Self {
        Self {
            location,
            steps,
            path,
        }
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
        Some(self.cmp(other))
    }
}

pub struct Map {
    map: Box<[u8]>,
    start: usize,
    columns: usize,
    rows: usize,
    connections: HashMap<usize, [Option<usize>; 4]>,
    loop_path: Vec<usize>,
    path_indices: HashMap<usize, usize>,
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
            loop_path: vec![],
            path_indices: HashMap::new(),
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

    pub fn compute_connections(&mut self) {
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

    pub fn find_loop_path(&mut self) {
        let mut min_steps_to_location: HashMap<usize, usize> = HashMap::new();
        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        queue.push(State::new(self.start, 0, vec![self.start]));
        min_steps_to_location.insert(self.start, 0);

        // we want to keep track of I think the two longest paths
        // which would be the paths to the farthest location from both sides of the loop
        let mut longest_paths: Vec<Vec<usize>> = vec![vec![], vec![]];

        while let Some(state) = queue.pop() {
            for connection in self.connections[&state.location].iter().flatten() {
                let steps = state.steps + 1;
                let min_steps = min_steps_to_location
                    .entry(*connection)
                    .or_insert(usize::MAX);
                if steps < *min_steps {
                    let mut path = state.path.clone();
                    path.push(*connection);

                    for longest_path in &mut longest_paths {
                        if path.len() > longest_path.len() {
                            *longest_path = path.clone();
                            break;
                        }
                    }

                    *min_steps = steps;
                    queue.push(State::new(*connection, steps, path));
                }
            }
        }

        assert!(longest_paths[0] != longest_paths[1]);

        if longest_paths[0].last() == longest_paths[1].last() {
            longest_paths[1].pop();
        }

        self.loop_path.append(&mut longest_paths[0]);
        for location in longest_paths[1].iter().skip(1).rev() {
            self.loop_path.push(*location);
        }

        for (path_index, &map_index) in self.loop_path.iter().enumerate() {
            self.path_indices.insert(map_index, path_index);
        }
    }

    pub fn steps_to_farthest_part_of_loop(&self) -> usize {
        self.loop_path.len() / 2
    }

    pub fn tiles_inside_loop(&self) -> usize {
        let mut inside_tiles: usize = 0;
        let mut pipe_stack: Vec<u8> = vec![];
        let mut inside = false;

        for (i, tile) in self.map.iter().enumerate() {
            if self.column(i) == 0 {
                inside = false;
                pipe_stack.clear();
            }

            // we're inside the path
            if self.tile_in_loop_path(i) {
                // this is the first path tile we're encountering after non-path tiles
                if pipe_stack.is_empty() {
                    pipe_stack.push(*tile);
                } else {
                    // we were previously on a path tile, so check if this is a continuation or not
                    if self.adjacent_in_path(i, i - 1) {
                        pipe_stack.push(*tile);
                    } else {
                        // we're moving out of one part of the path into another part
                        // so we want to inspect the pipe stack to see if we crossed the path
                        if self.crossed_path(&pipe_stack) {
                            inside = !inside;
                        }
                        pipe_stack.clear();
                        pipe_stack.push(*tile);
                    }
                }
            } else {
                // we're transitioning from the path to a non-path space
                if !pipe_stack.is_empty() && self.crossed_path(&pipe_stack) {
                    inside = !inside;
                }

                if inside {
                    inside_tiles += 1;
                }

                pipe_stack.clear();
            }
        }

        inside_tiles
    }

    fn tile_in_loop_path(&self, i: usize) -> bool {
        self.path_indices.contains_key(&i)
    }

    fn adjacent_in_path(&self, a_index: usize, b_index: usize) -> bool {
        let path_len = self.loop_path.len();
        let a_path_index = self.path_indices[&a_index];

        let lesser_index = (a_path_index + path_len - 1) % path_len;
        let greater_index = (a_path_index + 1) % path_len;

        self.loop_path[lesser_index] == b_index || self.loop_path[greater_index] == b_index
    }

    fn crossed_path(&self, pipe_stack: &[u8]) -> bool {
        let last_tile = pipe_stack.last().unwrap();
        let previous_tiles = &pipe_stack[..(pipe_stack.len() - 1)];

        let came_from_below = previous_tiles.contains(&b'F')
            || (previous_tiles.contains(&b'S') && self.path_goes_down_from_start());

        let came_from_above = previous_tiles.contains(&b'L')
            || (previous_tiles.contains(&b'S') && self.path_goes_up_from_start());

        match last_tile {
            b'|' => true,
            b'J' => came_from_below, // came from below, exited above
            b'7' => came_from_above, // came from above, exited below
            b'S' => {
                match (
                    self.path_goes_up_from_start(),
                    self.path_goes_down_from_start(),
                ) {
                    (true, true) => true,                            // S is effectively a |
                    (true, false) => previous_tiles.contains(&b'F'), // S is effectively a J
                    (false, true) => previous_tiles.contains(&b'L'), // S is effectively a 7
                    (false, false) => unreachable!(),                // invalid row ending char
                }
            }
            _ => unreachable!(),
        }
    }

    fn path_goes_up_from_start(&self) -> bool {
        self.row(self.start) > 0 && self.adjacent_in_path(self.start, self.start - self.columns)
    }

    fn path_goes_down_from_start(&self) -> bool {
        self.row(self.start) < self.rows - 1
            && self.adjacent_in_path(self.start, self.start + self.columns)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_state_ordering() {
        let first = State::new(2, 1, vec![]);
        let second = State::new(1, 2, vec![]);
        let third = State::new(2, 2, vec![]);

        assert!(first > second);
        assert!(second > third);
    }

    #[test]
    fn test_adjacent_in_path() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");

        let mut map = Map::new(&input);
        map.compute_connections();
        map.find_loop_path();

        // the loop path looks like this
        // [10, 11, 6, 7, 2, 3, 8, 13, 14, 19, 18, 17, 16, 21, 20, 15]

        // adjacent at the beginning (both ways)
        assert!(map.adjacent_in_path(10, 11));
        assert!(map.adjacent_in_path(11, 10));

        // adjacent at the end (both ways)
        assert!(map.adjacent_in_path(20, 15));
        assert!(map.adjacent_in_path(15, 20));

        // since it's a loop, the ends are adjacent too
        assert!(map.adjacent_in_path(10, 15));
        assert!(map.adjacent_in_path(15, 10));

        // these aren't adjacent
        assert!(!map.adjacent_in_path(11, 7));
    }

    #[test]
    fn test_tiles_in_loop() {
        let inputs_and_outputs = [
            ("test-input.txt", 1),
            ("part-2-test-1.txt", 4),
            ("part-2-test-2.txt", 8),
            ("part-2-test-3.txt", 10),
        ];

        for (file, expected_tiles_enclosed) in inputs_and_outputs {
            let input = fs::read_to_string(file).unwrap();

            let mut map = Map::new(&input);
            map.compute_connections();
            map.find_loop_path();

            assert_eq!(map.tiles_inside_loop(), expected_tiles_enclosed);
        }
    }
}
