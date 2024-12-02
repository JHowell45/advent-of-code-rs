use core::{file_reader::get_file_contents};

use event_2015::shared::day3::{Location, VisitedHouses};

pub fn main() {
    let directions = get_file_contents(2015, 3);
    let mut santa = SantaLocation::new();
    santa.apply_directions(directions.as_str());
    println!("Total Houses: {}", santa.unique_houses_visited());
}

#[derive(Debug)]
pub struct SantaLocation {
    location: Location,
    visited: VisitedHouses,
}

impl SantaLocation {
    pub fn new() -> Self {
        Self {
            location: Location::new(),
            visited: VisitedHouses::new(),
        }
    }

    pub fn unique_houses_visited(&self) -> usize {
        self.visited.houses_visited()
    }

    pub fn apply_directions(&mut self, directions: &str) {
        for direction in directions.chars().into_iter() {
            self.move_house(direction);
        }
    }

    pub fn move_house(&mut self, direction: char) {
        self.location.add_direction(direction);
        self.visited
            .has_visited(self.location.create_location_key());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    #[case(">^^v^<>v<<<v<v^>>v", 13)]
    fn examples(#[case] directions: &str, #[case] houses_visited: usize) {
        let mut santa = SantaLocation::new();
        santa.apply_directions(directions);
        println!("{:#?}", santa);
        assert_eq!(santa.unique_houses_visited(), houses_visited);
    }
}
