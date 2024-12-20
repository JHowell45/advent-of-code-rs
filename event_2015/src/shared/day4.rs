pub fn lowest_hex(secret: &str, n: usize) -> usize {
    let mut index: usize = 1;
    let comp: String = vec!["0"; n].into_iter().collect();
    let mut hex = generate_hex(format!("{}{}", secret, index).as_str());
    while &hex[..n] != comp {
        index += 1;
        hex = generate_hex(format!("{}{}", secret, index).as_str());
    }
    return index;
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
