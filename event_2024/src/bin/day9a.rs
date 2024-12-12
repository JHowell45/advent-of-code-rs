use core::file_reader::get_file_contents;

use event_2024::shared::day9::DiskMap;

fn main() {
    let mut disk = DiskMap::from_map(get_file_contents(2024, 9).as_str());
    disk.defragment();
    println!("Filesystem Checksum: {}", disk.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402", 1928)]
    fn example(#[case] disk_map: &str, #[case] expected_checksum: u64) {
        let mut map = DiskMap::from_map(disk_map);
        map.defragment();
        assert_eq!(map.checksum(), expected_checksum);
    }
}
