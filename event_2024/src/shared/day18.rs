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
    space: usize,
    bytes: Vec<Point>
}

impl MemorySpace {
    pub fn new(range: usize, bytes: Vec<Point>) -> Self {
        Self {
            space: range,
            bytes: bytes
        }
    }

    pub fn display_space(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}