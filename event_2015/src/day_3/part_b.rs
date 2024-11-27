use super::shared::SantaLocation;

pub fn part_b() {}

struct RoboSantaAndSantaLocations {
    santa: SantaLocation,
    robo_santa: SantaLocation
}

impl RoboSantaAndSantaLocations {
    pub fn new() -> Self {
        Self {
            santa: SantaLocation::new(),
            robo_santa: SantaLocation::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn examples() {}
}
