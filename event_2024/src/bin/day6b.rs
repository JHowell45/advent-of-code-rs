use core::file_reader::get_file_contents;

use event_2024::shared::day6::PatrolMap;

fn main() {
    let mut map: PatrolMap = PatrolMap::from_string(&get_file_contents(2024, 6));
    println!(
        "Viable obstruction positions: {}",
        map.viable_obstruction_positions()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        6
    )]
    fn example(#[case] map: &str, #[case] unique_obstructions: usize) {
        let mut map = PatrolMap::from_string(map);
        println!("{map:?}");
        assert_eq!(map.viable_obstruction_positions(), unique_obstructions);
    }
}
