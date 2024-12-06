use std::collections::HashSet;

fn main() {}

#[derive(Debug)]
enum MapState {
    Empty,
    Obstruction,
    Guard,
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
    guard_positions: HashSet<(usize, usize)>,
}

impl PatrolMap {
    pub fn from_string(map: &str) -> Self {
        let mut current_guard_pos: (usize, usize) = (0, 0);
        let mut guard_direction: GuardDirection = GuardDirection::North;
        let mut local_map: Vec<Vec<MapState>> = Vec::new();
        let mut guard_positions: HashSet<(usize, usize)> = HashSet::new();

        for (y, row) in map.lines().into_iter().enumerate() {
            let mut r: Vec<MapState> = Vec::new();
            for (x, point) in row.chars().into_iter().enumerate() {
                match point {
                    '.' => r.push(MapState::Empty),
                    '#' => r.push(MapState::Obstruction),
                    _ => {
                        r.push(MapState::Guard);
                        current_guard_pos = (x, y);
                        guard_positions.insert((x, y));

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
            map: local_map,
            current_guard_pos: current_guard_pos,
            current_guard_direction: guard_direction,
            guard_positions: HashSet::new(),
        }
    }

    pub fn get_guard_unique_positions(&mut self) -> usize {
        0
    } 

    fn interate(&self) {}

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
        assert_eq!(map.get_guard_unique_positions(), unique_points);
    }
}
