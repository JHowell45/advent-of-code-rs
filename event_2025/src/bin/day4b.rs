use aoc_core::file_reader::get_file_contents;
use event_2025::shared::day4::Grid;

fn main() {
    let mut grid: Grid = Grid::from_str(get_file_contents(2025, 4).as_str());
    println!(
        "Total number of rolls accessible by forklift: {}",
        grid.recursive_accessible_rolls()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        43
    )]
    fn example(#[case] input: &str, #[case] expected: u32) {
        let mut grid: Grid = Grid::from_str(input);
        assert_eq!(grid.recursive_accessible_rolls(), expected);
    }
}
