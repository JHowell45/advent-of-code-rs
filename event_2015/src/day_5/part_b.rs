use core::{enums::Part, file_reader::get_file_contents};
use std::collections::HashSet;

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
struct LetterRepeat {
    pub characters: [Option<char>; 3],
}

impl LetterRepeat {
    pub fn new() -> Self {
        Self {
            characters: [None, None, None],
        }
    }

    pub fn add(&mut self, letter: char) {
        for i in 0..self.characters.len() {
            let v: Option<char> = self.characters[i];
            if v.is_none() {
                self.characters[i] = Some(letter);
                return;
            }
        }
        self.rotate_and_add(letter);
    }

    fn rotate_and_add(&mut self, letter: char) {
        for i in 1..self.characters.len() {
            self.characters[i - 1] = self.characters[i];
        }
        self.characters[2] = Some(letter);
    }

    pub fn check(&self) -> bool {
        self.characters[0] == self.characters[2]
    }
}

#[derive(Debug)]
struct NiceString {
    string: String,
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
        let mut letter_repeat: LetterRepeat = LetterRepeat::new();

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
                        false => {
                            pairs_lookup.insert(pair.clone());
                        }
                    }
                }

                prior_pair = pair;
            }

            if !spaced_letter_repeat {
                letter_repeat.add(letter);
                spaced_letter_repeat = letter_repeat.check();
            }

            prior = Some(letter);
        }

        Self {
            string: String::from(string),
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

    #[rstest]
    fn letter_repeat_add_char() {
        let mut repeat = LetterRepeat::new();
        repeat.add('a');
        assert_eq!(repeat.check(), false);
        assert_eq!(repeat.characters[0], Some('a'));
        assert_eq!(repeat.characters[1], None);
        assert_eq!(repeat.characters[2], None);
    }

    #[rstest]
    fn letter_repeat_add_two_chars() {
        let mut repeat = LetterRepeat::new();
        repeat.add('a');
        repeat.add('b');
        assert_eq!(repeat.check(), false);
        assert_eq!(repeat.characters[0], Some('a'));
        assert_eq!(repeat.characters[1], Some('b'));
        assert_eq!(repeat.characters[2], None);
    }

    #[rstest]
    fn letter_repeat_add_three_chars() {
        let mut repeat = LetterRepeat::new();
        repeat.add('a');
        repeat.add('b');
        repeat.add('c');
        assert_eq!(repeat.check(), false);
        assert_eq!(repeat.characters[0], Some('a'));
        assert_eq!(repeat.characters[1], Some('b'));
        assert_eq!(repeat.characters[2], Some('c'));
    }

    #[rstest]
    fn letter_repeat_add_four_chars() {
        let mut repeat = LetterRepeat::new();
        repeat.add('a');
        repeat.add('b');
        repeat.add('c');
        repeat.add('d');
        assert_eq!(repeat.check(), false);
        assert_eq!(repeat.characters[0], Some('b'));
        assert_eq!(repeat.characters[1], Some('c'));
        assert_eq!(repeat.characters[2], Some('d'));
    }

    #[rstest]
    fn letter_repeat_passes_check() {
        let mut repeat = LetterRepeat::new();
        repeat.add('a');
        repeat.add('b');
        repeat.add('a');
        assert_eq!(repeat.check(), true);
    }
}
