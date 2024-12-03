use core::file_reader::get_file_contents;
use regex::Regex;


fn main() {
    let mut total: usize = 0;
    let puzzle_data: String = get_file_contents(2024, 3);
    let pattern = Regex::new("(mul\\(\\d{1,3},\\d{1,3}\\))").unwrap();
    for (_, [path, lineno, line]) in pattern.captures_iter(&puzzle_data).map(|c| c.extract()) {
        println!("{path:} | {lineno:} | {line:}");
    }
    println!("Sum of multiplications: {total:}");
}