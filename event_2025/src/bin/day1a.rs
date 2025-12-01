

use aoc_core::file_reader::get_file_contents;

fn parse_rotation(rotation: &str) -> i32 {
    &rotation[1..].parse::<i32>().unwrap()
        * match rotation.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid rotation type!!"),
        }
}

fn rotate(current_position: i32, rotation: i32) -> i32 {
    (current_position + rotation).rem_euclid(100)
}

pub fn main() {
    let mut current_point: i32 = 50;
    let mut result = 0;
    for rotation in get_file_contents(2025, 1).lines().into_iter() {
        current_point = rotate(current_point, parse_rotation(rotation));
        if current_point == 0 {
            result += 1;
        }
    }
    println!("Total times landed on zero: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("L68", -68)]
    #[case("R68", 68)]
    fn test_parse_rotation(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(parse_rotation(input), expected);
    }

    #[rstest]
    #[case(50, "L68", 82)]
    #[case(82, "L30", 52)]
    #[case(52, "R48", 0)]
    #[case(0, "L5", 95)]
    #[case(95, "R60", 55)]
    #[case(55, "L55", 0)]
    #[case(0, "L1", 99)]
    #[case(99, "L99", 0)]
    #[case(0, "R14", 14)]
    #[case(14, "L82", 32)]
    fn example(#[case] start: i32, #[case] rotation: &str, #[case] expected: i32) {
        assert_eq!(rotate(start, parse_rotation(rotation)), expected);
    }
}
