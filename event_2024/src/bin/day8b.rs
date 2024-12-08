use core::file_reader::get_file_contents;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let map = FrequencyMap::from_map(get_file_contents(2024, 8).as_str());
    println!("Unique antinode locations: {}", map.unique_antinode_locations());
}


struct FrequencyMap {
    antennna_locations: HashMap<char, HashSet<(i32, i32)>>,
    x_dimension: usize,
    y_dimension: usize,
}

impl FrequencyMap {
    pub fn from_map(map: &str) -> Self {
        let mut antennna_locations: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
        let mut x: usize = 0;
        let mut y: usize = map.lines().try_len().unwrap();
        for (y_idx, row) in map.lines().into_iter().enumerate() {
            x = row.chars().try_len().unwrap();
            for (idx, node) in row.chars().into_iter().enumerate() {
                let point = (idx, y_idx);
                match node {
                    '.' => {}
                    _ => match antennna_locations.get_mut(&node) {
                        Some(antennas) => {
                            antennas.insert(point);
                        }
                        None => {
                            antennna_locations.insert(node, HashSet::from([point]));
                        }
                    },
                }
            }
        }
        Self {
            antennna_locations,
            x_dimension: x,
            y_dimension: y,
        }
    }

    pub fn unique_antinode_locations(&self) -> usize {
        let antinode_locations: HashSet<(i32, i32)> = HashSet::new();
        return antinode_locations.len();
    }

    fn calculate_distance(&self, from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        ((to_x - from_x), (to_y - from_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", 14)]
    fn example(#[case] input: &str, #[case] unique_locations: usize) {}
}
