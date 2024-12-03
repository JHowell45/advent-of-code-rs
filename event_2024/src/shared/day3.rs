use regex::Regex;

pub fn mul_sum(puzzle_data: &str) -> i32 {
    let pattern = Regex::new("(mul\\((?<a>\\d{1,3}),(?<b>\\d{1,3})\\))").unwrap();
    pattern.captures_iter(&puzzle_data).map(|caps| {
        let a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
        let b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
        a * b
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161)]
    fn example(#[case] text: &str, #[case] sum: i32) {
        assert_eq!(mul_sum(text), sum);
    }
}