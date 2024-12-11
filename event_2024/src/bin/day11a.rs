use core::file_reader::get_file_contents;

use event_2024::shared::day11::Stones;

fn main() {
    let blinks: usize = 25;
    let mut stones = Stones::from_string(get_file_contents(2024, 11).as_str());
    println!("Total stones after {blinks} blinks: {}", stones.total_stones(blinks));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("5 62914 65 972 0 805922 6521 1639064", 25, 199753)]
    fn correct_result(#[case] puzzle: &str, #[case] blinks: usize, #[case] expected: usize) {
        let mut stones = Stones::from_string(puzzle);
        assert_eq!(stones.total_stones(blinks), expected);
    }
}
