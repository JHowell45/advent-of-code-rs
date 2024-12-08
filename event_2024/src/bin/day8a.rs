use core::file_reader::get_file_contents;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::Display, iter::repeat_n, ops::{Add, Sub}};

use itertools::Itertools;

fn main() {
    let map = FrequencyMap::from_map(get_file_contents(2024, 8).as_str());
    map.display_map(None);
    println!("Unique antinode locations: {}", map.unique_antinode_locations());
}

#[derive(Debug, Copy, Clone, Hash)]
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

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self::new(x as i32, y as i32)
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

impl PartialOrd for Point {
    fn ge(&self, other: &Self) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    fn gt(&self, other: &Self) -> bool {
        self.x > other.x && self.y > other.y
    }

    fn le(&self, other: &Self) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    fn lt(&self, other: &Self) -> bool {
        self.x < other.x && self.y < other.y
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            return Some(std::cmp::Ordering::Equal);
        }
        return None;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


struct FrequencyMap {
    antennna_locations: HashMap<char, HashSet<Point>>,
    max_dimension: Point,
}

impl FrequencyMap {
    pub fn from_map(map: &str) -> Self {
        let mut antennna_locations: HashMap<char, HashSet<Point>> = HashMap::new();
        let mut x: usize = 0;
        let mut y: usize = 0;
        for (y_idx, row) in map.lines().into_iter().enumerate() {
            if x == 0 {
                x = row.chars().collect::<Vec<char>>().len();
            }
            for (idx, node) in row.chars().into_iter().enumerate() {
                let point = Point::from_usize(idx, y_idx);
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
            max_dimension: Point::from_usize(x, y)
        }
    }

    pub fn unique_antinode_locations(&self) -> usize {
        let mut antinode_locations: HashSet<Point> = HashSet::new();
        for (k, points) in self.antennna_locations.iter() {
            println!("{k:}");
            for x in repeat_n(points.iter(), 2).multi_cartesian_product() {
                let (a, b) = (x[0], x[1]);
                if a != b {
                    let d = b.clone() - a.clone();
                    let antinode_a: Point = a.clone() - d.clone();
                    let antinode_b: Point = b.clone() + d.clone();

                    if antinode_a >= Point::origin() && antinode_a <= self.max_dimension {
                        antinode_locations.insert(antinode_a);
                    }
                    if antinode_b >= Point::origin() && antinode_b <= self.max_dimension {
                        antinode_locations.insert(antinode_b);
                    }

                    println!("\t{a:} -> {b:} == {d:}");
                    println!("\t Antinode A: {antinode_a:}");
                    println!("\tAntinode B: {antinode_b:}");
                    
                }
            }
        }
        self.display_map(Some(antinode_locations.clone()));
        return antinode_locations.len();
    }

    pub fn display_map(&self, antinodes: Option<HashSet<Point>>) {
        for y in 0..self.max_dimension.y {
            for x in 0..self.max_dimension.x {
                let p = Point::new(x, y);
                if let Some(ref check) = antinodes {
                    if check.contains(&p) {
                        print!("#");
                        continue;
                    }
                }
                match self.slow_antenna_check(p) {
                    Some(c) => print!("{c:}"),
                    None => print!("."),
                }
            }
            println!();
        }
        println!();
    }

    fn slow_antenna_check(&self, point: Point) -> Option<char> {
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
    #[case("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............", 14)]
    fn example(#[case] input: &str, #[case] unique_locations: usize) {
        let map = FrequencyMap::from_map(input);
        map.display_map(None);
        println!("Max Dim: {}", map.max_dimension);
        assert_eq!(map.unique_antinode_locations(), unique_locations);
    }
}
