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
    #[case(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        34
    )]
    fn example(#[case] input: &str, #[case] unique_locations: usize) {
        let map = FrequencyMap::from_map(input);
        map.display_map(None);
        println!("Max Dim: {}", map.max_dimension);
        assert_eq!(map.inline_antinode_locations(), unique_locations);
    }
}