use std::collections::HashMap;

pub struct SantaLocation {
    x: i32,
    y: i32,
    all_houses_visited: usize,
    visited: HashMap<String, usize>,
}

impl SantaLocation {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            all_houses_visited: 0,
            visited: HashMap::new(),
        }
    }

    pub fn houses_with_n_presents(&self, n: usize) -> usize {
        let mut count = 0;
        for (_, v) in self.visited.iter() {
            if *v == n {
                count += 1;
            }
        }
        return count;
    }

    pub fn move_house(&mut self, direction: char) {
        match direction {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            '^' => self.y -= 1,
            'v' => self.y += 1,
            _ => panic!("Invalid direction! {}", direction)
        }
        self.all_houses_visited += 1;
        self.has_visited();
    }

    pub fn location(&self) -> [i32; 2] {
        [self.x, self.y]
    }

    fn has_visited(&mut self) {
        let key = self.create_location_key();
        if self.visited.contains_key(&key) {
            *self.visited.get_mut(&key).unwrap() += 1;
        } else {
            self.visited.insert(key.clone(), 1);
        }
    }

    fn create_location_key(&self) -> String {
        return format!("{}{}", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::no_panic('>', [1, 0])]
    #[case::no_panic('<', [-1, 0])]
    #[case::no_panic('^', [0, -1])]
    #[case::no_panic('v', [0, 1])]
    #[should_panic(expected="Invalid direction! q")]
    #[case::panic_with_message("q", [0, 0])]
    fn test_move_house(#[case] direction: char, #[case] location: [i32; 2]) {
        let mut santa = SantaLocation::new();
        santa.move_house(direction);
        assert_eq!(santa.location(), location);
    }
}
