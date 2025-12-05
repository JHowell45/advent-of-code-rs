use aoc_core::file_reader::get_file_contents;

enum GridMarker {
    Empty,
    Paper,
    Reachable,
}

pub struct Grid {
    map: Vec<GridMarker>,
    pub x_size: usize,
    pub y_size: usize,
}
impl Grid {
    pub fn from_str(map: &str) -> Self {
        Self {
            map: map
                .trim()
                .chars()
                .into_iter()
                .map(|c| match c {
                    '.' => GridMarker::Empty,
                    '@' => GridMarker::Paper,
                    _ => panic!("Invalid map marker!"),
                })
                .collect(),
            x_size: map.lines().nth(1).iter().len(),
            y_size: map.lines().collect::<Vec<&str>>().len(),
        }
    }

    pub fn accessible_rolls(&self) -> u32 {
        self.map
            .iter()
            .enumerate()
            .map(|(idx, m)| match m {
                GridMarker::Paper | &GridMarker::Reachable => self.check_marker(idx) as u32,
                GridMarker::Empty => 0
            })
            .sum()
    }

    fn check_marker(&self, index: usize) -> bool {
        false
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
