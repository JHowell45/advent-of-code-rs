use core::file_reader::get_file_contents;

use event_2024::shared::day9::DiskMap;

fn main() {
    let mut disk = DiskMap::from_map(get_file_contents(2024, 9).as_str());
    disk.old_defragment_files();
    println!("Filesystem Checksum: {}", disk.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402", 2858)]
    fn example(#[case] disk_map: &str, #[case] expected_checksum: u64) {
        let mut map: DiskMap = DiskMap::from_map(disk_map);
        map.defragment_files();
        println!("{:?}", map.files);
        println!("{}", map.formatted_disk());
        assert_eq!(map.checksum(), expected_checksum);
    }
}
