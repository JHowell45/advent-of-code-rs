pub fn part_a() {}

fn lowest_hex(secret: &str) -> usize {
    let mut index: usize = 0;

    return index;
}

fn hex_starts_with_n_zeros(hex: &str, n: usize) -> bool {

}

fn generate_hex(secret: &str) -> String {
    format!("{:x}", md5::compute(secret))
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