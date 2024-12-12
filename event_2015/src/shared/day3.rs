use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn add_direction(&mut self, direction: char) {
        match direction {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => panic!("Invalid direction! {}", direction),
        }
    }

    pub fn create_location_key(&self) -> String {
        format!("{}{}", self.x, self.y)
    }

    pub fn location(&self) -> [i32; 2] {
        [self.x, self.y]
    }
}

#[derive(Debug)]
pub struct VisitedHouses {
    visited: HashMap<String, usize>,
}

impl Default for VisitedHouses {
    fn default() -> Self {
        Self::new()
    }
}

impl VisitedHouses {
    pub fn new() -> Self {
        let mut visited: HashMap<String, usize> = HashMap::new();
        visited.insert(String::from("00"), 1);
        Self { visited }
    }

    pub fn houses_visited(&self) -> usize {
        self.visited.keys().count()
    }

    pub fn has_visited(&mut self, key: String) {
        if self.visited.contains_key(&key) {
            *self.visited.get_mut(&key).unwrap() += 1;
        } else {
            self.visited.insert(key.clone(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::no_panic('>', [1, 0])]
    #[case::no_panic('<', [-1, 0])]
    #[case::no_panic('^', [0, 1])]
    #[case::no_panic('v', [0, -1])]
    #[should_panic(expected = "Invalid direction! q")]
    #[case::panic_with_message("q", [0, 0])]
    fn move_house(#[case] direction: char, #[case] expected_location: [i32; 2]) {
        let mut location = Location::new();
        location.add_direction(direction);
        assert_eq!(location.location(), expected_location);
    }
}
