use aoc_core::file_reader::get_file_contents;

fn check_number_repetitions(number: &str) -> bool {}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("11", true)]
    #[case("12", false)]
    #[case("13", false)]
    #[case("14", false)]
    #[case("15", false)]
    #[case("16", false)]
    #[case("22", true)]
    #[case("1010", true)]
    #[case("123123", true)]
    #[case("222222", true)]
    #[case("123123123", false)]
    fn test_check_number_repetitions(#[case] value: &str, #[case] expected: bool) {
        assert_eq!(check_number_repetitions(value), expected);
    }

    #[rstest]
    fn example() {}
}
