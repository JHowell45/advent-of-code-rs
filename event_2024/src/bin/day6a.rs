use core::file_reader::get_file_contents;
use std::{collections::HashSet, thread::sleep, time::Duration};

fn main() {
    let mut map: PatrolMap = PatrolMap::from_string(&get_file_contents(2024, 6));
    println!("The gaurd distinct positions are: {}", map.get_guard_unique_positions());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    current_guard_pos: (i32, i32),
    current_guard_direction: GuardDirection,
    max_x: i32,
    max_y: i32
}

impl PatrolMap {
    pub fn from_string(map: &str) -> Self {
        let mut current_guard_pos: (i32, i32) = (0, 0);
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
                        current_guard_pos = (x as i32, y as i32);

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
 
        Self {
            max_x: local_map[0].len() as i32,
            max_y: local_map.len() as i32,
            map: local_map,
            current_guard_pos: current_guard_pos,
            current_guard_direction: guard_direction,
        }
    }

    pub fn get_guard_unique_positions(&mut self) -> usize {
        while self.interate() {
            self.display_map();
            sleep(Duration::from_secs(2));
        }
        self.all_guard_points()
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
        println!();
    }

    fn interate(&mut self) -> bool {
        let (x, y) = self.current_guard_pos;
        let (next_x, next_y) = match self.current_guard_direction {
            GuardDirection::North => (x, y-1),
            GuardDirection::East => (x + 1, y),
            GuardDirection::South => (x, y + 1),
            GuardDirection::West => (x - 1, y),
        };
        println!("{:?}", self.current_guard_pos);
        println!("{next_x:}, {next_y:}");
        println!("{:?}", self.get_point(next_x, next_y));

        if self.guard_outside_boundaries((next_x, next_y)) {
            return false;
        }
        match self.get_point(next_x, next_y) {
            MapState::Empty => {
                self.set_point(next_x, next_y, MapState::GuardRoute);
                self.current_guard_pos = (next_x, next_y);
            }
            _ => {}
        }
        return true;
    }

    fn get_point(&self, x: i32, y: i32) -> MapState {
        self.map[y as usize][x as usize]
    }

    fn set_point(&mut self, x: i32, y: i32, state: MapState) {
        self.map[y as usize][x as usize] = state;
    }

    fn guard_outside_boundaries(&self, point: (i32, i32)) -> bool {
        let (x, y) = point;
        (x < 0 || x > self.max_x - 1) || (y < 0 || y > self.max_y - 1)
    }

    fn all_guard_points(&self) -> usize {
        let mut count = 0;
        for row in self.map.iter() {
            for p in row.iter() {
                if *p == MapState::GuardRoute {
                    count += 1;
                }
            }
        }
        count
    }
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
