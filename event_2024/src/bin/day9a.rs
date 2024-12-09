use core::file_reader::get_file_contents;

use event_2024::shared::day9::DiskMap;

fn main() {
    let disk = DiskMap::from_map(get_file_contents(2024, 9).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402")]
    fn example(#[case] disk_map: &str) {
        let map = DiskMap::from_map(disk_map);
        println!("{map:#?}");
        println!("{}", map.formatted_disk());
    }
}