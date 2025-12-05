use aoc_core::file_reader::get_file_contents;
use event_2025::shared::day3::Bank;

fn maximum_bank_joltage_sum(banks: String) -> u64 {
    banks
        .lines()
        .into_iter()
        .map(|batteries| Bank::new(batteries).max_joltage_n(2))
        .sum()
}

fn main() {
    println!(
        "Sum of the maximum voltage for each bank: {}",
        maximum_bank_joltage_sum(get_file_contents(2025, 3))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "987654321111111
811111111111119
234234234234278
818181911112111",
        357
    )]
    fn example(#[case] banks: &str, #[case] maximum_joltage_sum: u64) {
        assert_eq!(
            maximum_bank_joltage_sum(String::from(banks)),
            maximum_joltage_sum
        );
    }
}
