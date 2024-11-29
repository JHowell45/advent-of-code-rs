use std::collections::HashSet;

pub static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
pub static INVALID_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

#[derive(Debug)]
pub struct NiceString {
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
                if invalid_lookup.contains(String::from(format!("{}{}", prior_letter, letter)).as_str()) {
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
