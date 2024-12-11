use core::file_reader::get_file_contents;

use event_2024::shared::day11::Stones;

fn main() {
    let mut stones = Stones::from_string(get_file_contents(2024, 11).as_str());
    let x: u32 = 345564;
    let l = x.checked_ilog10().unwrap() + 1;
    let l = x.checked_ilog10().unwrap() + 1;
    println!("{x:}: {l:}, {}", x / (10_i32.pow(l / 2) as u32));
    // println!("Total stones after 25 blinks: {}", stones.total_stones(75));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
