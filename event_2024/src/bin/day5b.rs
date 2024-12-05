use core::file_reader::get_file_contents;

use event_2024::shared::day5::LaunchSafetyManual;

fn main() {
    let content = get_file_contents(2024, 5);
    let manual = LaunchSafetyManual::from_string(&content);
    println!("Sum of middle pages: {}", manual.fix_and_sum_middle());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
