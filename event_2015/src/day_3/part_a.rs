use core::{enums::Part, file_reader::get_file_contents};

use super::shared::SantaLocation;



pub fn part_a() {
    let directions = get_file_contents(2015, 3, Part::A);
    let mut santa = SantaLocation::new();
    santa.apply_directions(directions.as_str());
    println!("Total Houses: {}", santa.unique_houses_visited());
}

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
