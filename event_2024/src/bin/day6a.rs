use core::file_reader::get_file_contents;

use event_2024::shared::day6::PatrolMap;

fn main() {
    let mut map: PatrolMap = PatrolMap::from_string(&get_file_contents(2024, 6));
    println!("The guard distinct positions are: {}", map.get_guard_unique_positions());
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...", 41)]
    fn example(#[case] map: &str, #[case] unique_points: usize) {
        let mut map = PatrolMap::from_string(map);
        println!("{map:?}");
        assert_eq!(map.get_guard_unique_positions(), unique_points);
    }
}
