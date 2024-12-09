use core::file_reader::get_file_contents;

use event_2024::shared::day9::DiskMap;

fn main() {
    let disk = DiskMap::from_map(get_file_contents(2024, 9).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // #[rstest]
    // #[case("2333133121414131402", "")]
    // fn example(#[case] disk_map: &str, #[case] expected_disk: &str) {
    //     let mut map = DiskMap::from_map(disk_map);
    //     println!("{map:#?}");
    //     assert_eq!(map.formatted_disk(), expected_disk);
    //     map.defragment();
    //     println!("{}", map.formatted_disk());
    // }
}
