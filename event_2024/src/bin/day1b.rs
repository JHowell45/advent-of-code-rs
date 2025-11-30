use aoc_core::file_reader::get_file_contents;

use event_2024::shared::day1::LocationSearch;

pub fn main() {
    let search = LocationSearch::parse_input(get_file_contents(2024, 1).as_str());
    println!("The similarity score is: {}", search.similarity_score());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 31)]
    fn example(#[case] locations: &str, #[case] similarity: usize) {
        assert_eq!(
            LocationSearch::parse_input(locations).similarity_score(),
            similarity
        );
    }
}
