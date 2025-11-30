use aoc_core::file_reader::get_file_contents;

use event_2024::shared::day7::{sum_of_valid_results, Operator};

fn main() {
    println!(
        "Total calibration result: {}",
        sum_of_valid_results(
            get_file_contents(2024, 7).as_str(),
            vec![Operator::Add, Operator::Multiply, Operator::Concat]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
