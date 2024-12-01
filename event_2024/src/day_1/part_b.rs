use core::{enums::Part, file_reader::get_file_contents};

use super::shared::LocationSearch;

pub fn part_b() {
    let search = LocationSearch::parse_input(get_file_contents(2024, 1, Part::A).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 31)]
    fn example(#[case] locations: &str, #[case] similarity: usize) {
        let mut search = LocationSearch::parse_input(locations);
        assert_eq!(search.total_distances(), similarity);
    }
}
