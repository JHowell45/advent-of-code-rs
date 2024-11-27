use core::{enums::Part, file_reader::get_file_contents};

use super::shared::{Location, VisitedHouses};

pub fn part_b() {
    let directions = get_file_contents(2015, 3, Part::A);
    let mut santa = RoboSantaAndSantaLocations::new();
    santa.apply_directions(directions.as_str());
    println!("Total Houses: {}", santa.unique_houses_visited());
}

struct RoboSantaAndSantaLocations {
    santa: Location,
    robo_santa: Location,
    swap_santa: bool,
    visited: VisitedHouses,
}

impl RoboSantaAndSantaLocations {
    pub fn new() -> Self {
        Self {
            santa: Location::new(),
            robo_santa: Location::new(),
            swap_santa: false,
            visited: VisitedHouses::new(),
        }
    }

    pub fn unique_houses_visited(&self) -> usize {
        self.visited.houses_visited()
    }

    pub fn apply_directions(&mut self, directions: &str) {
        for direction in directions.chars().into_iter() {
            match self.swap_santa {
                false => {
                    self.santa.add_direction(direction);
                    self.visited.has_visited(self.santa.create_location_key());
                }
                true => {
                    self.robo_santa.add_direction(direction);
                    self.visited.has_visited(self.robo_santa.create_location_key());
                }
            }
            self.swap_santa = !self.swap_santa;
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
