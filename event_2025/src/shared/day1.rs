pub fn parse_rotation(rotation: &str) -> i32 {
    &rotation[1..].parse::<i32>().unwrap()
        * match rotation.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid rotation type!!"),
        }
}

#[cfg(test)]
mod tests {
    use super::parse_rotation;
    use rstest::rstest;

    #[test]
    fn test_works() {
        assert_eq!(1, 1);
    }

    #[rstest]
    #[case("L68", -68)]
    #[case("R68", 68)]
    fn test_parse_rotation(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(parse_rotation(input), expected);
    }
}