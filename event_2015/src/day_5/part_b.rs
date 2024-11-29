use core::{enums::Part, file_reader::get_file_contents};
use std::collections::HashMap;

pub fn part_b() {
    let mut nice_strings: usize = 0;
    for string in get_file_contents(2015, 5, Part::A).lines().into_iter() {
        if is_nice_string(string) {
            nice_strings += 1;
        }
    }
    println!("The number of nice strings: {}", nice_strings);
}

fn is_nice_string(string: &str) -> bool {
    let nice = NiceString::parse(string);
    println!("{:#?}", nice);
    nice.results()
}

#[derive(Debug)]
struct NiceString {
    overlapping_pairs: bool,
    pairs_count: HashMap<String, usize>,
    spaced_letter_repeat: bool
}

impl NiceString {
    pub fn parse(string: &str) -> Self {
        let mut overlapping_pairs = false;
        let mut pairs_count: HashMap<String, usize> = HashMap::new();
        let mut spaced_letter_repeat: bool = false;

        let mut prior: Option<char> = None;

        for letter in string.chars().into_iter() {
            prior = Some(letter);
        }

        Self {
            overlapping_pairs, pairs_count, spaced_letter_repeat
        }
    }

    pub fn results(&self) -> bool {
        !self.overlapping_pairs && self.spaced_letter_repeat
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
