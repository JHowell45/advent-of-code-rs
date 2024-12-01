pub struct LocationIds {
    left_locations: [i32; 1000],
    right_locations: [i32; 1000],
}

impl LocationIds {
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
}
