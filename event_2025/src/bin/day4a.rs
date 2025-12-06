use aoc_core::file_reader::get_file_contents;
use event_2025::shared::day4::Grid;

fn main() {
    let grid: Grid = Grid::from_str(get_file_contents(2025, 4).as_str());
    println!(
        "Total number of rolls accessible by forklift: {}",
        grid.accessible_rolls()
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
        13
    )]
    fn example(#[case] paper_grid: &str, #[case] expected: u32) {
        let grid: Grid = Grid::from_str(paper_grid);
        println!("{:?}\n", grid);
        grid.debug_map();
        assert_eq!(Grid::from_str(paper_grid).accessible_rolls(), expected);
    }
}
