use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Map {
    data: Vec<Vec<char>>,
    start: Point,
    end: Point
}

impl Map {
    pub fn from_string(map: &str) -> Self {
        let mut data: Vec<Vec<char>> = Vec::new();
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;

        for (y_idx, row) in map.lines().enumerate() {
            let mut row_data: Vec<char> = Vec::new();
            for (x_idx, c) in row.chars().into_iter().enumerate() {
                match c {
                    'S' => start = Some(Point::new(x_idx, y_idx)),
                    'E' => end = Some(Point::new(x_idx, y_idx)),
                    _ => {},
                }
                row_data.push(c);
            }
            data.push(row_data);
        }
        if start.is_none() || end.is_none() {
            panic!("No start or end found!!");
        }
        Self {
            data: data,
            start: start.unwrap(),
            end: end.unwrap()
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::from("\n"); 
        for y in self.data.iter() {
            s.push_str(y.iter().collect::<String>().as_str());
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct PathFinder {
    map: Map,
    score: usize,
    player_position: Point,
    player_direction: Direction,
}

impl PathFinder {
    pub fn from_string(map: &str) -> Self {
        let map: Map = Map::from_string(map);
        Self {
            player_direction: Direction::East,
            player_position: map.start.clone(),
            map: map,
            score: 0,
        }
    }

    pub fn lowest_point_path(&mut self) -> usize {
        while self.player_position != self.map.end {
            self.move_next();
            println!("{self:#?}");
        }
        self.score
    }

    fn move_next(&mut self) {
        let next: Point = match self.player_direction {
            Direction::North => Point::new(self.player_position.x, self.player_position.y + 1),
            Direction::South => Point::new(self.player_position.x, self.player_position.y - 1),
            Direction::East => Point::new(self.player_position.x + 1, self.player_position.y),
            Direction::West => Point::new(self.player_position.x - 1, self.player_position.y),
        };
        if !self.move_forward(next) {
            self.rotate();
        }
    }

    fn move_forward(&mut self, next: Point) -> bool {
        if self.map.data[next.y][next.x] == '.' {
            return false;
        }
        self.map.data[self.player_position.y][self.player_position.x] = 'X';
        self.player_position = next;
        self.score += 1;
        return true;
    }

    fn rotate(&mut self) {
        self.score += 1000;
        self.player_direction = match self.player_direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
", 7036)]
    fn example(#[case] map: &str, #[case] score: usize) {
        let mut path_finder: PathFinder = PathFinder::from_string(map);
        println!("{path_finder:?}");
        assert_eq!(path_finder.lowest_point_path(), score);
    }
}