use std::{thread::sleep, time::Duration};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MapState {
    Empty,
    Obstruction,
    GuardRoute,
    GuardRouteVertical,
    GuardRouteHorizontal,
    CustomObstruction,
}

pub enum IterateState {
    Continue,
    Loop,
    Exit,
}

#[derive(Debug)]
pub enum GuardDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct PatrolMap {
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
            // print!("{}[2J", 27 as char);
            // self.display_map();
            // sleep(Duration::from_millis(150));
        }
        self.all_guard_points()
    }

    fn interate(&mut self) -> bool {
        let (next_x, next_y) = self.get_next_point();
        if self.guard_outside_boundaries((next_x, next_y)) {
            return false;
        }
        match self.get_point(next_x, next_y) {
            MapState::Empty => {
                self.set_point(next_x, next_y, MapState::GuardRoute);
                self.current_guard_pos = (next_x, next_y);
            }
            MapState::Obstruction => self.rotate_guard(),
            MapState::GuardRoute => self.current_guard_pos = (next_x, next_y),
            _ => {}
        }
        return true;
    }

    pub fn viable_obstruction_positions(&mut self) -> usize {
        let viable_pos: usize = 0;
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.get_point(x, y) == MapState::Empty {
                    while let state = self.viable_obstructions_iterate() {
                        // print!("{}[2J", 27 as char);
                        // self.display_map();
                        // sleep(Duration::from_millis(150));
                    }   
                }
            }
        }
        return viable_pos;
    }

    fn viable_obstructions_iterate(&mut self) -> IterateState {
        let (next_x, next_y) = self.get_next_point();
        if self.guard_outside_boundaries((next_x, next_y)) {
            return IterateState::Exit;
        }
        match self.get_point(next_x, next_y) {
            MapState::Empty => {
                self.set_point(next_x, next_y, MapState::GuardRoute);
                self.current_guard_pos = (next_x, next_y);
            }
            MapState::Obstruction => self.rotate_guard(),
            MapState::GuardRoute => self.current_guard_pos = (next_x, next_y),
            _ => {}
        }
        return IterateState::Continue;
    }

    pub fn display_map(&self) {
        for row in self.map.iter() {
            for point in row.iter() {
                match point {
                    MapState::Empty => print!("."),
                    MapState::Obstruction => print!("#"),
                    MapState::GuardRoute => print!("X"),
                    _ => panic!("Unhandled state!"),
                }
            }
            println!();
        }
        println!();
    }

    fn get_next_point(&self) -> (i32, i32) {
        let (x, y) = self.current_guard_pos;
        match self.current_guard_direction {
            GuardDirection::North => (x, y-1),
            GuardDirection::East => (x + 1, y),
            GuardDirection::South => (x, y + 1),
            GuardDirection::West => (x - 1, y),
        }
    }
    fn rotate_guard(&mut self) {
        self.current_guard_direction = match self.current_guard_direction {
            GuardDirection::North => GuardDirection::East,
            GuardDirection::East => GuardDirection::South,
            GuardDirection::South => GuardDirection::West,
            GuardDirection::West => GuardDirection::North,
        }
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