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

    pub fn display_space(&self) {
        for y in self.space {
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
    #[case(6)]
}