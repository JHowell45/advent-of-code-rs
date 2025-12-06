use aoc_core::file_reader::get_file_contents;

#[derive(Debug)]
enum GridMarker {
    Empty,
    Paper,
    Reachable,
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
                GridMarker::Paper => (self.get_neighbour_count(idx) < 4) as u32,
                GridMarker::Empty => 0,
                GridMarker::Reachable => panic!("What's that doing there?"),
            })
            .sum()
    }

    pub fn debug_map(&self) {
        let debug_map = self.map.iter().enumerate().map(|(idx, m)| match m {
            GridMarker::Paper => {
                if self.get_neighbour_count(idx) < 4 {
                    GridMarker::Reachable
                } else {
                    GridMarker::Paper
                }
            }
            GridMarker::Empty => GridMarker::Empty,
            GridMarker::Reachable => GridMarker::Reachable,
        });
        for (idx, v) in debug_map.enumerate() {
            print!(
                "{}",
                match v {
                    GridMarker::Empty => ".",
                    GridMarker::Paper => "@",
                    GridMarker::Reachable => "x",
                }
            );
            if (idx + 1) % self.x_size == 0 {
                print!("\n");
            }
        }
    }

    pub fn get_neighbour_count(&self, index: usize) -> usize {
        let mut neighbours: usize = 0;
        let left_check: bool = index > 0 && index % self.x_size != 0;
        let right_check: bool = index % self.x_size < self.x_size - 1;
        let top_check: bool = index > self.x_size;
        let bottom_check: bool = index < (self.x_size * (self.y_size - 1));

        if left_check {
            println!("Should be empty!!");
            let left_index: usize = index - 1;
            neighbours += match self.map[left_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };
            if top_check {
                neighbours += match self.map[left_index - (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
            if bottom_check {
                neighbours += match self.map[left_index + (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
        }
        if right_check {
            let right_index: usize = index + 1;
            neighbours += match self.map[right_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };

            if top_check {
                neighbours += match self.map[right_index - (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
            if bottom_check {
                neighbours += match self.map[right_index + (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
        }
        if top_check {
            neighbours += match self.map[index - self.x_size] {
                GridMarker::Paper => 1,
                _ => 0,
            };
        }
        if bottom_check {
            neighbours += match self.map[index + self.x_size] {
                GridMarker::Paper => 1,
                _ => 0,
            };
        }
        return neighbours;
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
    #[case(2, 3)]
    #[case(3, 3)]
    #[case(6, 3)]
    #[case(7, 4)]
    #[case(12, 6)]
    #[case(10, 2)]
    fn example_neighbour_count(#[case] index: usize, #[case] expected_neighbours: usize) {
        assert_eq!(
            Grid::from_str(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            )
            .get_neighbour_count(index),
            expected_neighbours
        );
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
        println!("{:?}\n", grid);
        grid.debug_map();
        assert_eq!(Grid::from_str(paper_grid).accessible_rolls(), expected);
    }
}
