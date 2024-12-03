use core::file_reader::get_file_contents;
use regex::Regex;


fn main() {
    let puzzle_data: String = get_file_contents(2024, 3);
    println!("Sum of enabled multiplications: {}", enabled_sum(&puzzle_data));
}

fn enabled_sum(text: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48)]
    fn example(#[case] text: &str, #[case] sum: i32) {
        assert_eq!(enabled_sum(text), sum);
    }
}