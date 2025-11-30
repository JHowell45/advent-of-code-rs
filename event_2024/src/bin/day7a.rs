use aoc_core::file_reader::get_file_contents;

use event_2024::shared::day7::{sum_of_valid_results, Operator};

fn main() {
    println!(
        "Total calibration result: {}",
        sum_of_valid_results(
            get_file_contents(2024, 7).as_str(),
            vec![Operator::Add, Operator::Multiply]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_2024::shared::day7::{validate_equation, validate_equation_ops, Operator};
    use rstest::rstest;

    #[rstest]
    #[case(190, vec![10, 19], vec![&Operator::Add], false)]
    #[case(190, vec![10, 19], vec![&Operator::Multiply], true)]
    fn test_validate_equation_ops(
        #[case] result: i64,
        #[case] numbers: Vec<i64>,
        #[case] ops: Vec<&Operator>,
        #[case] expected: bool,
    ) {
        assert_eq!(validate_equation_ops(result, &numbers, &ops), expected);
    }

    #[rstest]
    #[case(190, vec![10, 19], true)]
    #[case(3267, vec![81, 40, 27], true)]
    #[case(83, vec![17, 5], false)]
    #[case(156, vec![15, 6], false)]
    #[case(7290, vec![6, 8, 6, 15], false)]
    #[case(161011, vec![16, 10, 13], false)]
    #[case(192, vec![17, 8, 14], false)]
    #[case(21037, vec![9, 7, 18, 13], false)]
    #[case(292, vec![11, 6, 16, 20], true)]
    fn test_validate_equation(
        #[case] result: i64,
        #[case] numbers: Vec<i64>,
        #[case] expected: bool,
    ) {
        assert_eq!(
            validate_equation(result, numbers, &vec![Operator::Add, Operator::Multiply]),
            expected
        );
    }

    #[rstest]
    #[case(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        3749
    )]
    fn example(#[case] data: &str, #[case] expected: i64) {
        assert_eq!(sum_of_valid_results(data, vec![Operator::Add, Operator::Multiply]), expected);
    }
}
