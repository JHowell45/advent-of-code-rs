use core::{enums::Part, file_reader::get_file_contents};
use std::collections::HashMap;


pub struct LocationSearch {
    left_locations: [i32; 1000],
    right_locations: [i32; 1000],
}

impl LocationSearch {
    pub fn new() -> Self {
        Self {
            left_locations: [0; 1000],
            right_locations: [0; 1000],
        }
    }

    pub fn parse_input(locations: &str) -> Self {
        let mut instance = Self::new();
        for (index, line) in locations.lines().into_iter().enumerate() {
            let inputs: Vec<&str> = line.split(" ").filter(|v| *v != "").collect();
            instance.add_left(index, inputs[0].trim().parse::<i32>().unwrap());
            instance.add_right(index, inputs[1].trim().parse::<i32>().unwrap());
        }
        return instance;
    }

    pub fn add_left(&mut self, index: usize, value: i32) {
        self.left_locations[index] = value;
    }

    pub fn add_right(&mut self, index: usize, value: i32) {
        self.right_locations[index] = value;
    }

    pub fn total_distances(&mut self) -> usize {
        let mut distance: i32 = 0;
        self.left_locations.sort();
        self.right_locations.sort();
        for (left, right) in self.left_locations.iter().zip(self.right_locations.iter()) {
            distance += (left - right).abs();
        }
        return distance as usize;
    }

    pub fn similarity_score(&self) -> usize {
        let mut similarity_score: i32 = 0;
        let mut lookup: HashMap<i32, i32> = HashMap::new();
        for number in self.right_locations.iter() {
            match lookup.get_mut(number) {
                Some(count) => *count += 1,
                None => {
                    lookup.insert(*number, 1);
                }
            }
        }
        for number in self.left_locations.iter() {
            if let Some(count) = lookup.get(number) {
                similarity_score += number * count;
            }
        }
        return similarity_score as usize;
    }
}

fn main() {
    let mut locations = LocationSearch::parse_input(get_file_contents(2024, 1, Part::A).as_str());
    println!("The total distance is: {}", locations.total_distances());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 11)]
    fn example(#[case] locations: &str, #[case] distance: usize) {
        let mut search: LocationSearch = LocationSearch::parse_input(locations);
        assert_eq!(search.total_distances(), distance);
    }
}
