use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    x: u8,
    y: u8
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

pub struct MemorySpace {
    range: usize,
    space: Vec<Vec<char>>,
    bytes: Vec<Point>
}

impl MemorySpace {
    pub fn new(range: usize, bytes: Vec<Point>) -> Self {
        Self {
            range: range,
            space: (0..=range).map(|_| (0..=range).map(|_| '.').collect()).collect(),
            bytes: bytes
        }
    }

    pub fn from_string(range: usize, puzzle: &str) -> Self {
        let mut bytes: Vec<Point> = Vec::new();
        let pattern = Regex::new(r"(?<x>\d+),(?<y>\d+)").unwrap();
        for (_, [x, y]) in pattern.captures_iter(puzzle).map(|c| c.extract()) {
            bytes.push(Point::new(x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap()));
        }
        Self::new(range, bytes)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(6, "5,4
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
2,0", 12, 22)]
    fn example(#[case] range: usize, #[case] input: &str, #[case] steps: usize, #[case] steps: usize) {
        let space = MemorySpace::
    }
}