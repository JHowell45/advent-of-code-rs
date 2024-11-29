pub fn part_b() {}

fn is_nice_string(string: &str) -> bool {
    true
}

#[derive(Debug)]
struct NiceString {

}

impl NiceString {
    pub fn parse(string: &str) -> Self {

    }

    pub fn results(&self) -> bool {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn examples(#[case] string: &str, #[case] is_nice: bool) {
        assert_eq!(is_nice_string(string), is_nice);
    }
}
