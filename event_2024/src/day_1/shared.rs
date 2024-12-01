pub struct LocationIds {
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
