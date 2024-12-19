use std::collections::HashSet;

pub struct Onsen {
    towels: HashSet<String>
}

impl Onsen {
    pub fn from_string(towels: &str) -> Self {
        Self {
            towels: HashSet::from_iter(towels.split(", ").map(|t| t.to_string()))
        }
    }

    pub fn validate_rack(&self, rack: &str) -> bool {
        let rack: Vec<char> = rack.chars().collect();
        let mut start_idx: usize = 0;
        let mut offset: usize = 1;
        while start_idx < rack.len() - 1 {
            println!("{:?}", rack[start_idx..start_idx + offset]);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("brwrr", true)]
    #[case("bggr", true)]
    #[case("gbbr", true)]
    #[case("rrbgbr", true)]
    #[case("ubwu", false)]
    #[case("bwurrg", true)]
    #[case("brgr", true)]
    #[case("bbrgwb", false)]
    fn test_validate_rack(#[case] rack: &str, #[case] expected_result: bool) {
        let onsen = Onsen::from_string("r, wr, b, g, bwu, rb, gb, br");
        assert_eq!(onsen.validate_rack(rack), expected_result);
    }
}