use core::file_reader::get_file_contents;
use std::{collections::{HashMap, HashSet}, iter::repeat_n, ops::{Add, Sub}};

use itertools::Itertools;

fn main() {
    let map = FrequencyMap::from_map(get_file_contents(2024, 8).as_str());
    map.display_map();
    println!("Unique antinode locations: {}", map.unique_antinode_locations());
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


struct FrequencyMap {
    antennna_locations: HashMap<char, HashSet<(i32, i32)>>,
    y_dimension: usize,
    y_dimension: usize,
}

impl FrequencyMap {
    pub fn from_map(map: &str) -> Self {
        let mut antennna_locations: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
        let mut x: usize = 0;
        let mut y: usize = 0;
        for (y_idx, row) in map.lines().into_iter().enumerate() {
            if x == 0 {
                x = row.chars().collect::<Vec<char>>().len();
            }
            for (idx, node) in row.chars().into_iter().enumerate() {
                let point = (idx as i32, y_idx as i32);
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
            y += 1;
        }
        Self {
            antennna_locations,
            x_dimension: x,
            y_dimension: y,
        }
    }

    pub fn unique_antinode_locations(&self) -> usize {
        let antinode_locations: HashSet<(i32, i32)> = HashSet::new();
        for points in self.antennna_locations.values() {
            for x in repeat_n(points.iter(), 2).multi_cartesian_product() {
                let (a, b) = (x[0], x[1]);
                if a != b {
                    println!("{a:?} -> {b:?} == {:?}", self.calculate_distance(*a, *b));
                }
            }
        }
        return antinode_locations.len();
    }

    fn calculate_distance(&self, from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        ((to_x - from_x), (to_y - from_y))
    }

    pub fn display_map(&self) {
        for y in 0..self.y_dimension {
            for x in 0..self.x_dimension {
                let point: (i32, i32) = (x as i32, y as i32);
                match self.slow_antenna_check(point) {
                    Some(c) => print!("{c:}"),
                    None => print!("."),
                }
            }
            println!();
        }
        println!();
    }

    fn slow_antenna_check(&self, point: (i32, i32)) -> Option<char> {
        for (k, points) in self.antennna_locations.iter() {
            if let Some(_) = points.get(&point) {
                return Some(*k);
            }
        }
        return None;
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
