use std::{thread, time};

#[derive(Debug, Clone, Copy)]
pub enum GridMarker {
    Empty,
    Paper,
    Reachable,
}

#[derive(Debug)]
pub struct Grid {
    pub map: Vec<GridMarker>,
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
                GridMarker::Paper => (self.get_neighbour_count(idx, &self.map) < 4) as u32,
                GridMarker::Empty => 0,
                GridMarker::Reachable => panic!("What's that doing there?"),
            })
            .sum()
    }

    pub fn recursive_accessible_rolls(&mut self) -> u32 {
        let mut rolls_removed: Option<u32> = None;
        let mut total_rolls: u32 = 0;
        let mut updated_map: Vec<GridMarker> = self.map.clone();

        while rolls_removed.is_none() || rolls_removed > Some(0) {
            let map_clone: Vec<GridMarker> = updated_map.clone();
            rolls_removed = Some(0);
            for (idx, marker) in updated_map.iter_mut().enumerate() {
                match marker {
                    GridMarker::Paper => {
                        let neighbours: usize = self.get_neighbour_count(idx, &map_clone.clone());
                        if neighbours < 4 {
                            rolls_removed = match rolls_removed {
                                None => Some(1),
                                Some(x) => Some(x + 1),
                            };
                            *marker = GridMarker::Reachable;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(rolls) = rolls_removed {
                total_rolls += rolls;
            }
        }
        return total_rolls;
    }

    pub fn display_debug_map(&self, map: &Vec<GridMarker>) {
        for (idx, v) in map.iter().enumerate() {
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
    pub fn get_neighbour_count(&self, index: usize, map: &Vec<GridMarker>) -> usize {
        let mut neighbours: usize = 0;
        let left_check: bool = index > 0 && index % self.x_size != 0;
        let right_check: bool = index % self.x_size < self.x_size - 1;
        let top_check: bool = index > self.x_size;
        let bottom_check: bool = index < (self.x_size * (self.y_size - 1));

        if left_check {
            let left_index: usize = index - 1;
            neighbours += match map[left_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };
            if top_check {
                neighbours += match map[left_index - (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
            if bottom_check {
                neighbours += match map[left_index + (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
        }
        if right_check {
            let right_index: usize = index + 1;
            neighbours += match map[right_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };

            if top_check {
                neighbours += match map[right_index - (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
            if bottom_check {
                neighbours += match map[right_index + (self.x_size)] {
                    GridMarker::Paper => 1,
                    _ => 0,
                }
            }
        }

        if top_check {
            let top_index: usize = index - self.x_size;
            neighbours += match map[top_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };
        }

        if bottom_check {
            let bottom_index: usize = index + self.x_size;
            neighbours += match map[bottom_index] {
                GridMarker::Paper => 1,
                _ => 0,
            };
        }
        return neighbours;
    }
}

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
    #[case(10, 3)]
    fn example_neighbour_count(#[case] index: usize, #[case] expected_neighbours: usize) {
        let grid: Grid = Grid::from_str(
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
        );
        assert_eq!(
            grid.get_neighbour_count(index, &grid.map),
            expected_neighbours
        );
    }
}
