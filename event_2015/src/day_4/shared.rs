pub fn lowest_hex(secret: &str, n: usize) -> usize {
    let mut index: usize = 1;
    while !hex_starts_with_n_zeros(&generate_hex(format!("{}{}", secret, index).as_str()), n) {
        index += 1;
    }
    return index;
}

fn hex_starts_with_n_zeros(hex: &str, n: usize) -> bool {
    let s: String = vec!["0"; n].into_iter().collect();
    return &hex[..n] == s.as_str();
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
        assert_eq!(lowest_hex(secret, 5), answer);
    }
}