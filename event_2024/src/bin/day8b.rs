use core::file_reader::get_file_contents;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let map = FrequencyMap::from_map(get_file_contents(2024, 8).as_str());
    println!("Unique antinode locations: {}", map.unique_antinode_locations());
}

struct Antenna {
    frequency: String,
    location: (usize, usize),
}
impl Antenna {
    pub fn new(frequency: String, location: (usize, usize)) -> Self {
        Self {
            frequency,
            location,
        }
    }
}

struct FrequencyMap {
    antennna_locations: HashMap<char, HashSet<(usize, usize)>>,
    x_dimension: usize,
    y_dimension: usize,
}

impl FrequencyMap {
    pub fn from_map(map: &str) -> Self {
        let mut antennna_locations: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
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
        let antinode_locations: HashSet<(usize, usize)> = HashSet::new();
        return antinode_locations.len();
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
