use aoc_core::file_reader::get_file_contents;

#[derive(Debug)]
enum GridMarker {
    Empty,
    Paper,
}

#[derive(Debug)]
pub struct Grid {
    map: Vec<GridMarker>,
    pub x_size: usize,
    pub y_size: usize,
}
impl Grid {
    pub fn from_str(map: &str) -> Self {
        Self {
            x_size: map.lines().nth(1).unwrap().chars().count(),
            y_size: map.lines().collect::<Vec<&str>>().len(),
            map: map
                .trim()
                .chars()
                .filter(|c| *c != '\n')
                .into_iter()
                .map(|c| match c {
                    '.' => GridMarker::Empty,
                    '@' => GridMarker::Paper,
                    _ => panic!("Invalid map marker '{}'!", c),
                })
                .collect(),
        }
    }

    pub fn accessible_rolls(&self) -> u32 {
        self.map
            .iter()
            .enumerate()
            .map(|(idx, m)| match m {
                GridMarker::Paper => self.can_access_marker(idx) as u32,
                GridMarker::Empty => 0,
            })
            .sum()
    }

    fn can_access_marker(&self, index: usize) -> bool {
        let mut neighbours: usize = 0;
        let left_check: bool = index > 0 && (index + 1) % self.x_size != 0;
        let top_check: bool = index >= self.x_size;
        let bottom_check: bool = index < (self.x_size * (self.y_size - 1));

        if left_check {
            neighbours += match self.map[index - 1] {
                GridMarker::Paper => 1,
                _ => 0,
            }
        }
        if top_check {
            neighbours += match self.map[index - self.x_size] {
                GridMarker::Paper => 1,
                _ => 0,
            }
        }
        if bottom_check {
            neighbours += match self.map[index + self.x_size] {
                GridMarker::Paper => 1,
                _ => 0,
            }
        }
        println!("Index: {} || Neighbours: {}", index, neighbours);
        neighbours < 4
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", (10, 10))]
    #[case("..
..
..", (2, 3))]
    fn test_grid_size(#[case] input: &str, #[case] expected: (usize, usize)) {
        let grid: Grid = Grid::from_str(input);
        assert_eq!((grid.x_size, grid.y_size), expected);
    }

    #[rstest]
    #[case(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        13
    )]
    fn example(#[case] paper_grid: &str, #[case] expected: u32) {
        let grid: Grid = Grid::from_str(paper_grid);
        println!("{:?}", grid);
        assert_eq!(Grid::from_str(paper_grid).accessible_rolls(), expected);
    }
}
