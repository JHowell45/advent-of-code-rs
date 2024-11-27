use std::collections::HashMap;

#[derive(Debug)]
pub struct Location {
    x: i32,
    y: i32
}

impl Location {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0
        }
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
pub struct SantaLocation {
    location: Location,
    visited: HashMap<String, usize>,
}

impl SantaLocation {
    pub fn new() -> Self {
        let mut visited: HashMap<String, usize> = HashMap::new();
        visited.insert(String::from("00"), 1);
        Self {
            location: Location::new(),
            visited: visited,
        }
    }

    pub fn unique_houses_visited(&self) -> usize {
        self.visited.keys().count()
    }

    pub fn apply_directions(&mut self, directions: &str) {
        for direction in directions.chars().into_iter() {
            self.move_house(direction);
        }
    }

    pub fn move_house(&mut self, direction: char) {
        self.location.add_direction(direction);
        self.has_visited(self.location.create_location_key());
    }

    fn has_visited(&mut self, key: String) {
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
