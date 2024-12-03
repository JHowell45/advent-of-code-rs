use regex::Regex;

pub fn mul_sum(puzzle_data: &str) -> i32 {
    let pattern = Regex::new("(mul\\((?<a>\\d{1,3}),(?<b>\\d{1,3})\\))").unwrap();
    pattern.captures_iter(&puzzle_data).map(|caps| {
        let a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
        let b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
        a * b
    }).sum()
}