use core::file_reader::get_file_contents;
use regex::Regex;


fn main() {
    let mut total: i32 = 0;
    let puzzle_data: String = get_file_contents(2024, 3);
    let pattern = Regex::new("(mul\\(\\d{1,3},\\d{1,3}\\))").unwrap();
    for data in pattern.find_iter(&puzzle_data).map(|c| c.as_str()) {
        let m = Regex::new("\\d{1,3},\\d{1,3}").unwrap().find(data).unwrap();
        let numbers: Vec<i32> = m.as_str().split(",").map(|n| n.trim().parse::<i32>().unwrap()).collect();
        total += numbers[0] * numbers[1];
    }
    println!("Sum of multiplications: {total:}");
}