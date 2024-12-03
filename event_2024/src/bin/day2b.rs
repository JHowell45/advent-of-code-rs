use core::file_reader::get_file_contents;

use event_2024::shared::day2::{is_safe, parse_report};

fn main() {
    let mut safe_reports: usize = 0;
    for report in get_file_contents(2024, 2).lines().into_iter() {
        if safe_dampener(report) {
            safe_reports += 1;
        }
    }
    println!("The number of safe reports is: {}", safe_reports);
}

fn safe_dampener(report: &str) -> bool {
    let report = parse_report(report);

    match is_safe(report.clone()) {
        Some(_) => {
            for i in 0..report.len() {
                let mut local_report = report.clone();
                local_report.remove(i);
                if is_safe(local_report).is_none() {
                    return true;
                }
            }
        }
        None => return true,
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("1 3 6 7 9", true)]
    fn example(#[case] report: &str, #[case] expected: bool) {
        assert_eq!(safe_dampener(report), expected);
    }
}
