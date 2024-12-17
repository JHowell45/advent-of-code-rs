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
    data: Vec<char>,
    y_size: usize,
    x_size: usize,
    start: Point,
    end: Point
}

impl Map {
    pub fn from_string(map: &str) -> Self {
        Self {

        }
    }
}

pub struct PathFinder {
    map: Map,
    score: usize
}

impl PathFinder {
    pub fn from_string(map: &str) -> Self {

    }

    fn move_forward(&mut self) {}

    fn rotate(&mut self) {}
}

#[cfg(test)]
mod tests {

}