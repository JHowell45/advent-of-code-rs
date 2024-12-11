use core::file_reader::get_file_contents;

use event_2024::shared::day11::Stones;

fn main() {
    let mut stones = Stones::from_string(get_file_contents(2024, 11).as_str());
    println!("Total stones after 25 blinks: {}", stones.total_stones(75));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
