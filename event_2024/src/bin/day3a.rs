use core::file_reader::get_file_contents;
use event_2024::shared::day3::mul_sum;


fn main() {
    let puzzle_data: String = get_file_contents(2024, 3);
    println!("Sum of multiplications: {}", mul_sum(&puzzle_data));
}