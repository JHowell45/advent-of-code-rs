use pathfinding::prelude::astar;
use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
}

pub struct MemorySpace {
    range: usize,
    space: Vec<Vec<char>>,
    bytes: Vec<Point>,
}

impl MemorySpace {
    pub fn new(range: usize, bytes: Vec<Point>) -> Self {
        Self {
            range: range,
            space: (0..=range)
                .map(|_| (0..=range).map(|_| '.').collect())
                .collect(),
            bytes: bytes,
        }
    }

    pub fn from_string(range: usize, puzzle: &str) -> Self {
        let mut bytes: Vec<Point> = Vec::new();
        let pattern = Regex::new(r"(?<x>\d+),(?<y>\d+)").unwrap();
        for (_, [x, y]) in pattern.captures_iter(puzzle).map(|c| c.extract()) {
            bytes.push(Point::new(
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap(),
            ));
        }
        Self::new(range, bytes)
    }

    pub fn least_steps(&mut self, bytes: usize) -> usize {
        let start = Point::new(0, 0);
        let end = Point::new(self.range, self.range);
        self.display_space();
        for idx in 0..bytes {
            let p = &self.bytes[idx];
            self.space[p.y][p.x] = '#';
        }
        self.display_space();
        let result = astar(
            &start,
            |p| self.point_successors(p),
            |p| p.distance(&end) / 3,
            |p| *p == end,
        );
        return result.unwrap().1 as usize;
    }

    pub fn display_space(&self) {
        for y in self.space.iter() {
            for c in y.iter() {
                print!("{c}");
            }
            println!();
        }
        println!();
    }

    fn point_successors(&self, p: &Point) -> Vec<(Point, u32)> {
        let mut valid_points: Vec<(Point, u32)> = Vec::new();
        if p.x > 0 && self.space[p.y][p.x - 1] != '#' {
            valid_points.push((Point::new(p.x - 1, p.y), 1));
        }
        if p.x < self.range && self.space[p.y][p.x + 1] != '#' {
            valid_points.push((Point::new(p.x + 1, p.y), 1));
        }
        if p.y > 0 && self.space[p.y - 1][p.x] != '#' {
            valid_points.push((Point::new(p.x, p.y - 1), 1));
        }
        if p.y < self.range && self.space[p.y + 1][p.x] != '#' {
            valid_points.push((Point::new(p.x, p.y + 1), 1));
        }
        return valid_points;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        6,
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        12,
        22
    )]
    fn example(
        #[case] range: usize,
        #[case] input: &str,
        #[case] bytes: usize,
        #[case] steps: usize,
    ) {
        let mut space = MemorySpace::from_string(range, input);
        assert_eq!(space.least_steps(bytes), steps);
    }
}
