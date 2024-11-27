use std::collections::HashMap;

use super::shared::Location;

pub fn part_b() {}

struct RoboSantaAndSantaLocations {
    santa: Location,
    robo_santa: Location,
    swap_santa: bool,
    visited: HashMap<String, usize>,
}

impl RoboSantaAndSantaLocations {
    pub fn new() -> Self {
        Self {
            santa: Location::new(),
            robo_santa: Location::new(),
            swap_santa: false,
            visited: HashMap::new(),
        }
    }

    pub fn unique_houses_visited(&self) -> usize {
        self.visited.keys().count()
    }

    pub fn apply_directions(&mut self, directions: &str) {
        for direction in directions.chars().into_iter() {
            match self.swap_santa {
                false => {
                    self.santa.add_direction(direction);
                }
                true => {
                    self.robo_santa.add_direction(direction);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn examples(#[case] directions: &str, #[case] houses_visited: usize) {
        let mut santas = RoboSantaAndSantaLocations::new();
        santas.apply_directions(directions);
        assert_eq!(santas.unique_houses_visited(), houses_visited);
    }
}
