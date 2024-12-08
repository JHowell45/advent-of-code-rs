use core::file_reader::get_file_contents;

use event_2024::shared::day8::FrequencyMap;

fn main() {
    let map = FrequencyMap::from_map(get_file_contents(2024, 8).as_str());
    map.display_map(None);
    println!(
        "Unique antinode locations: {}",
        map.inline_antinode_locations()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
