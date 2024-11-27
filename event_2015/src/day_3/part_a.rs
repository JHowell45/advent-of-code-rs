

pub fn part_a() {}

#[cfg(test)]
mod tests {
    use crate::day_3::shared::SantaLocation;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn part_a_examples(
        #[case] directions: &str,
        #[case] houses_visited: usize,
    ) {
        let mut santa = SantaLocation::new();
        santa.apply_directions(directions);
        println!("{:#?}", santa);
        assert_eq!(santa.unique_houses_visited(), houses_visited);
    }
}
