use core::file_reader::get_file_contents;
use event_2024::shared::day3::mul_sum;
use regex::Regex;


fn main() {
    let puzzle_data: String = get_file_contents(2024, 3);
    println!("Sum of enabled multiplications: {}", enabled_sum(&puzzle_data));
}

fn enabled_sum(text: &str) -> i32 {
    let mut total: i32 = 0;
    let mut flag: bool = false;
    let mut start: usize = 0;
    let pattern = Regex::new("(do\\(\\))|(don't\\(\\))").unwrap();

    for enabled in pattern.find_iter(&text) {
        match flag {
            true => {
                if enabled.as_str() == "do()" {
                    flag = !flag;
                    start = enabled.end();
                }
            },
            false => {
                if enabled.as_str() == "don't()" {
                    flag = !flag;
                    total += mul_sum(&text[start..enabled.end()]);
                }
            }
        }
    }
    if !flag {
        total += mul_sum(&text[start..]);
    }
    return total;
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