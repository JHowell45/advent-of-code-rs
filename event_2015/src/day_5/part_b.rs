use core::{enums::Part, file_reader::get_file_contents};
use std::collections::{HashSet};

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
    pair_check: bool,
    spaced_letter_repeat: bool,
}

impl NiceString {
    pub fn parse(string: &str) -> Self {
        let mut overlapping_pairs = false;
        let mut pair_check: bool = false;
        let mut spaced_letter_repeat: bool = false;

        let mut pairs_lookup: HashSet<String> = HashSet::new();

        let mut prior: Option<char> = None;
        let mut prior_pair: String = String::from("");

        for letter in string.chars().into_iter() {
            if let Some(prior_letter) = prior {
                let pair = format!("{}{}", prior_letter, letter);
                if prior_pair == pair {
                    overlapping_pairs = true;
                    break;
                }
                if !pair_check {
                    match pairs_lookup.contains(&pair) {
                        true => pair_check = true,
                        false => {pairs_lookup.insert(pair.clone());},
                    }
                }
                prior_pair = pair;
            }
            prior = Some(letter);
        }

        Self {
            overlapping_pairs,
            pair_check,
            spaced_letter_repeat,
        }
    }

    pub fn results(&self) -> bool {
        !self.overlapping_pairs && self.spaced_letter_repeat && self.pair_check
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
