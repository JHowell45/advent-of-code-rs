use core::{file_reader::get_file_contents};
use std::collections::HashSet;

pub fn main() {
    let mut nice_strings: usize = 0;
    for string in get_file_contents(2015, 5).lines().into_iter() {
        if is_nice_string(string) {
            nice_strings += 1;
        }
    }
    println!("The number of nice strings: {}", nice_strings);
}

fn is_nice_string(string: &str) -> bool {
    let nice = NiceString::parse(string);
    nice.result()
}

pub static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
pub static INVALID_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

#[derive(Debug)]
struct NiceString {
    vowels: i32,
    contains_double_letter: bool,
    contains_invalid_str: bool,
}

impl NiceString {
    pub fn parse(string: &str) -> Self {
        let vowel_lookup: HashSet<char> = HashSet::from(VOWELS);
        let invalid_lookup: HashSet<&str> = HashSet::from(INVALID_STRINGS);

        let mut vowels: i32 = 0;
        let mut double_letter: bool = false;
        let mut invalid_str: bool = false;
        let mut prior: Option<char> = None;

        for letter in string.chars().into_iter() {
            if let Some(prior_letter) = prior {
                if invalid_lookup
                    .contains(String::from(format!("{}{}", prior_letter, letter)).as_str())
                {
                    invalid_str = true;
                    break;
                }
                if prior_letter == letter {
                    double_letter = true;
                }
            }
            if vowel_lookup.contains(&letter) {
                vowels += 1;
            }
            prior = Some(letter);
        }
        Self {
            vowels: vowels,
            contains_double_letter: double_letter,
            contains_invalid_str: invalid_str,
        }
    }

    pub fn result(&self) -> bool {
        self.vowels >= 3 && self.contains_double_letter && !self.contains_invalid_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn examples(#[case] string: &str, #[case] is_nice: bool) {
        assert_eq!(is_nice_string(string), is_nice);
    }
}
