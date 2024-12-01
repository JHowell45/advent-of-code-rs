use core::{enums::Part, file_reader::get_file_contents};

pub fn part_a() {
    let mut locations = LocationIds::new();
    for (index, location_pairs) in get_file_contents(2024, 1, Part::A).lines().into_iter().enumerate() {
        let (left, right) = parse_line(location_pairs);
        locations.add_left(index, left);
        locations.add_right(index, right);
    }
    println!("The total distance is: {}", locations.total_distances());
}

fn parse_line(line: &str) -> (i32, i32) {
    let inputs: Vec<&str> = line.split(" ").collect();
    (inputs[0].trim().parse::<i32>().unwrap(), inputs[1].trim().parse::<i32>().unwrap())
}

struct LocationIds {
    left_locations: [i32; 1000],
    right_locations: [i32; 1000],
}

impl LocationIds {
    pub fn new() -> Self {
        Self {
            left_locations: [0; 1000],
            right_locations: [0; 1000]
        }
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
        return distance as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}