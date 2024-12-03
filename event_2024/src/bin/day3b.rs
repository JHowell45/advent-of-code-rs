use core::file_reader::get_file_contents;
use regex::Regex;


fn main() {
    // let mut total: i32 = 0;
    let puzzle_data: String = get_file_contents(2024, 3);
    let pattern = Regex::new("(mul\\((?<a>\\d{1,3}),(?<b>\\d{1,3})\\))").unwrap();
    // for (a, b) in pattern.captures_iter(&puzzle_data).map(|caps| {
    //     let a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
    //     let b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
    //     (a, b)
    // }) {
    //     total += a * b;
    // }
    let total: i32 = pattern.captures_iter(&puzzle_data).map(|caps| {
        let a = caps.name("a").unwrap().as_str().parse::<i32>().unwrap();
        let b = caps.name("b").unwrap().as_str().parse::<i32>().unwrap();
        a * b
    }).sum();
    println!("Sum of enabled multiplications: {total:}");
}