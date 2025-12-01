use aoc_core::file_reader::get_file_contents;

use event_2025::shared::day1::parse_rotation;

fn calculate_clicks(curr: i32, rot: i32) -> u32 {
    let temp = curr + rot;
    let mut res = (temp / 100).abs();
    if curr != 0 && temp <= 0 {
        res += 1;
    }
    println!("curr = {curr} | rotate = {rot} | res = {res} | temp = {temp}");
    return res.try_into().unwrap();
}

fn rotate(current_position: i32, rotation: i32) -> (i32, u32) {
    let current_pos: i32 = (current_position + rotation).rem_euclid(100);

    (current_pos, calculate_clicks(current_position, rotation))
}

pub fn main() {
    let mut current_point: i32 = 50;
    let mut clicks: u32;
    let mut result: u32 = 0;
    for rotation in get_file_contents(2025, 1).lines().into_iter() {
        (current_point, clicks) = rotate(current_point, parse_rotation(rotation));
        result += clicks;
    }
    println!("Total times landed on zero: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(50, "L68", (82, 1))]
    #[case(82, "L30", (52, 0))]
    #[case(52, "R48", (0, 1))]
    #[case(0, "L5", (95, 0))]
    #[case(95, "R60", (55, 1))]
    #[case(55, "L55", (0, 1))]
    #[case(0, "L1", (99, 0))]
    #[case(99, "L99", (0, 1))]
    #[case(0, "R14", (14, 0))]
    #[case(14, "L82", (32, 1))]
    // Additional example case:
    #[case(50, "R1000", (50, 10))]
    fn test_rotate(#[case] start: i32, #[case] rotation: &str, #[case] expected: (i32, u32)) {
        assert_eq!(rotate(start, parse_rotation(rotation)), expected);
    }
}
