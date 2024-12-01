use core::{enums::Part, file_reader::get_file_contents};

use crate::day_1::shared::LocationIds;

pub fn part_a() {
    let mut locations = LocationIds::parse_input(get_file_contents(2024, 1, Part::A).as_str());
    println!("The total distance is: {}", locations.total_distances());
}

fn parse_line(line: &str) -> (i32, i32) {
    let inputs: Vec<&str> = line.split(" ").filter(|v| *v != "").collect();
    (
        inputs[0].trim().parse::<i32>().unwrap(),
        inputs[1].trim().parse::<i32>().unwrap(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3   4\n4   3\n2   5\n1   3\n3   9\n3   3", 11)]
    fn example(#[case] locations: &str, #[case] distance: usize) {
        let mut search = LocationIds::new();
        for (index, line) in locations.lines().into_iter().enumerate() {
            let (left, right) = parse_line(line);
            search.add_left(index, left);
            search.add_right(index, right);
        }
        assert_eq!(search.total_distances(), distance);
    }
}
