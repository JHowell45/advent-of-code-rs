use core::file_reader::get_file_contents;
use std::collections::HashSet;

fn main() {
    let mut map: PatrolMap = PatrolMap::from_string(&get_file_contents(2024, 6));
    println!("The gaurd distinct positions are: {}", map.get_guard_unique_positions());
}

#[derive(Debug)]
enum MapState {
    Empty,
    Obstruction,
    GuardRoute,
}

#[derive(Debug)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct PatrolMap {
    map: Vec<Vec<MapState>>,
    current_guard_pos: (usize, usize),
    current_guard_direction: GuardDirection,
    max_x: usize,
    max_y: usize
}

impl PatrolMap {
    pub fn from_string(map: &str) -> Self {
        let mut current_guard_pos: (usize, usize) = (0, 0);
        let mut guard_direction: GuardDirection = GuardDirection::North;
        let mut local_map: Vec<Vec<MapState>> = Vec::new();

        for (y, row) in map.lines().into_iter().enumerate() {
            let mut r: Vec<MapState> = Vec::new();
            for (x, point) in row.chars().into_iter().enumerate() {
                match point {
                    '.' => r.push(MapState::Empty),
                    '#' => r.push(MapState::Obstruction),
                    _ => {
                        r.push(MapState::GuardRoute);
                        current_guard_pos = (x, y);

                        guard_direction = match point {
                            '^' => GuardDirection::North,
                            '>' => GuardDirection::North,
                            '<' => GuardDirection::North,
                            'v' => GuardDirection::North,
                            _ => panic!("Invalid guard direction '{}'!", point)
                        }
                    }
                }
            }
            local_map.push(r);
        }
        let max_x = &local_map[0].len();
        let max_y = &local_map.len();
        Self {
            map: local_map,
            current_guard_pos: current_guard_pos,
            current_guard_direction: guard_direction,
            max_x: *max_x,
            max_y: *max_y,
        }
    }

    pub fn get_guard_unique_positions(&mut self) -> usize {
        0
    }

    pub fn display_map(&self) {
        for row in self.map.iter() {
            for point in row.iter() {
                match point {
                    MapState::Empty => print!("."),
                    MapState::Obstruction => print!("#"),
                    MapState::GuardRoute => print!("X"),
                }
            }
            println!();
        }
    }

    fn interate(&mut self) {
        let (x, y) = self.current_guard_pos;
        let (next_x, next_y) = match self.current_guard_direction {
            GuardDirection::North => (x, y - 1),
            GuardDirection::East => (x + 1, y),
            GuardDirection::South => (x, y + 1),
            GuardDirection::West => (x - 1, y),
        };
    }

    fn add_guard_position(&self) {}

    fn guard_outside_boundaries(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...", 41)]
    fn example(#[case] map: &str, #[case] unique_points: usize) {
        let mut map = PatrolMap::from_string(map);
        println!("{map:?}");
        map.display_map();
        assert_eq!(map.get_guard_unique_positions(), unique_points);
    }
}
