use core::file_reader::get_file_contents;

use event_2024::shared::day5::LaunchSafetyManual;

fn main() {
    let content = get_file_contents(2024, 5);
    let manual = LaunchSafetyManual::from_string(&content);
    println!("Sum of middle pages: {}", manual.sum_middle_values());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        143
    )]
    fn example(#[case] input: &str, #[case] expected: i32) {
        let manual = LaunchSafetyManual::from_string(&input);
        assert_eq!(manual.sum_middle_values(), expected);
    }
}
