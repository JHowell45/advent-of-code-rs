use aoc_core::file_reader::get_file_contents;
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
    NiceString::parse(string).results()
}

#[derive(Debug)]
struct LetterRepeat {
    pub characters: [Option<char>; 3],
    pub len: usize,
}

impl LetterRepeat {
    pub fn new() -> Self {
        Self {
            characters: [None, None, None],
            len: 3,
        }
    }

    pub fn add(&mut self, letter: char) {
        for i in 0..self.len {
            let v: Option<char> = self.characters[i];
            if v.is_none() {
                self.characters[i] = Some(letter);
                return;
            }
        }
        self.rotate_and_add(letter);
    }

    fn rotate_and_add(&mut self, letter: char) {
        for i in 1..self.len {
            self.characters[i - 1] = self.characters[i];
        }
        self.characters[self.len - 1] = Some(letter);
    }

    pub fn check(&self) -> bool {
        if self.characters[0] == self.characters[self.len - 1] {
            return true;
        }
        return false;
    }

    pub fn valid_pair(&self) -> Option<String> {
        if self.characters[self.len - 1].is_none() {
            if self.characters[0].is_some() && self.characters[1].is_some() {
                return Some(format!(
                    "{}{}",
                    self.characters[0].unwrap(),
                    self.characters[1].unwrap()
                ));
            }
        } else {
            return Some(format!(
                "{}{}",
                self.characters[1].unwrap(),
                self.characters[2].unwrap()
            ));
        }
        return None;
    }
}

#[derive(Debug)]
struct NiceString {
    pair_check: bool,
    spaced_letter_repeat: bool,
}

impl NiceString {
    pub fn parse(string: &str) -> Self {
        let mut pair_check: bool = false;
        let mut spaced_letter_repeat: bool = false;

        let mut pairs_lookup: HashSet<String> = HashSet::new();
        let mut letter_repeat: LetterRepeat = LetterRepeat::new();

        let mut prior_pair: Option<String> = None;

        for letter in string.chars().into_iter() {
            letter_repeat.add(letter);
            if !spaced_letter_repeat {
                spaced_letter_repeat = letter_repeat.check();
            }

            if !pair_check {
                if let Some(pair) = letter_repeat.valid_pair() {
                    match prior_pair {
                        Some(prior) => {
                            if prior != pair {
                                if pairs_lookup.contains(&pair) {
                                    pair_check = true
                                } else {
                                    pairs_lookup.insert(pair.clone());
                                }
                                prior_pair = Some(pair.clone());
                            } else {
                                prior_pair = None;
                            }
                        }
                        None => {
                            if pairs_lookup.contains(&pair) {
                                pair_check = true
                            } else {
                                pairs_lookup.insert(pair.clone());
                            }
                            prior_pair = Some(pair.clone());
                        }
                    }
                }
            }
        }

        Self {
            pair_check,
            spaced_letter_repeat,
        }
    }

    pub fn results(&self) -> bool {
        self.spaced_letter_repeat && self.pair_check
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
    #[case("aaaa", true)]
    #[case("aaabcb", false)]
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
