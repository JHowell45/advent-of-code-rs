pub fn part_a() {}

fn lowest_hex(secret: &str) -> usize {

}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn examples(#[case] secret: &str, #[case] answer: usize) {
        assert_eq!(lowest_hex(secret), answer);
    }
}