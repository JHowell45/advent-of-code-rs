use core::{file_reader::get_file_contents};

use event_2024::shared::day2::{is_safe, parse_report};



fn main() {
    let mut safe_reports: usize = 0;
    for report in get_file_contents(2024, 2).lines().into_iter() {
        if is_safe(parse_report(report)).is_none() {
            safe_reports += 1;
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", None)]
    #[case("1 2 7 8 9", Some(2))]
    #[case("9 7 6 2 1", Some(3))]
    #[case("1 3 2 4 5", Some(2))]
    #[case("8 6 4 4 1", Some(3))]
    #[case("1 3 6 7 9", None)]
    #[case("65 67 70 72 74 73", Some(5))]
    #[case("32 35 37 39 39", Some(4))]
    fn example(#[case] report: &str, #[case] expected: Option<usize>) {
        assert_eq!(is_safe(parse_report(report)), expected);
    }
}
