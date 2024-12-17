#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
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
            if start.is_none() || end.is_none() {
                panic!("No start or end found!!");
            }
        }
        Self {
            data: data,
            start: start.unwrap(),
            end: end.unwrap()
        }
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

    fn move_forward(&mut self) {
        self.score += 1;
    }

    fn rotate(&mut self) {
        self.score += 1000;
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
")]
    fn example(#[case] map: &str) {
        let path_finder: PathFinder = PathFinder::from_string(map);
        println!("{path_finder:?}");
    }
}