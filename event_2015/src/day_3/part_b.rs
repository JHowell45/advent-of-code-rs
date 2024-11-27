use super::shared::SantaLocation;

pub fn part_b() {}

struct RoboSantaAndSantaLocations {
    santa: SantaLocation,
    robo_santa: SantaLocation,
    swap_santa: bool
}

impl RoboSantaAndSantaLocations {
    pub fn new() -> Self {
        Self {
            santa: SantaLocation::new(),
            robo_santa: SantaLocation::new(),
            swap_santa: false
        }
    }

    pub fn apply_directions(&mut self, directions: &str) {
        for direction in directions.chars().into_iter() {

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
    fn examples(#[case] directions: &str, #[case] houses_visited: usize) {}
}
