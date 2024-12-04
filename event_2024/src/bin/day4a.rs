use core::file_reader::get_file_contents;

fn main() {
    let search = WordSearch::from_string(get_file_contents(2024, 4));
    println!(
        "How many times does 'XMAS' appear: {}",
        search.word_count("XMAS")
    );
}

#[derive(Debug)]
struct WordSearch {
    pub letters: Vec<char>,
    pub columns: usize,
}

impl WordSearch {
    pub fn from_string(text: String) -> Self {
        Self {
            columns: text.find("\n").unwrap(),
            letters: text.chars().filter(|c| *c != '\n').collect(),
        }
    }

    pub fn word_count(&self, word: &str) -> usize {
        let mut count: usize = 0;
        let _first_char: char = word.chars().next().unwrap();

        for (index, _c) in self.letters.iter().enumerate() {
            if let Some(local_word) = self.left(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.right(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.top(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_top_left(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_top_right(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.bottom(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_bottom_left(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_bottom_right(index, word) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
        }
        println!("{:?}", self);
        return count;
    }

    pub fn left(&self, index: usize, word_length: usize) -> Option<String> {
        if index >= word_length - 1 {
            let idx = index - (word_length - 1);
            let local_word: String = self.letters[idx..=index].iter().rev().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn right(&self, index: usize, word: &str) -> Option<String> {
        if index % self.columns < self.columns - word.len() + 1 {
            let local_word: String = self.letters[index..index + word.len()].iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn top(&self, index: usize, word: &str) -> Option<String> {
        if (index as i32) - (self.columns * (word.len() - 1)) as i32 + 1 > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                chars.insert(i, self.letters[index - (self.columns * i)]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn bottom(&self, index: usize, word: &str) -> Option<String> {
        if index + self.columns * (word.len() - 1) < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                chars.insert(i, self.letters[index + (self.columns * i)]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn diagonal_top_left(&self, index: usize, word: &str) -> Option<String> {
        if (index as i32) - (self.columns * (word.len() - 1)) as i32 - (word.len() as i32) - 2 > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                chars.insert(i, self.letters[index - (self.columns * i) - i]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn diagonal_top_right(&self, index: usize, word: &str) -> Option<String> {
        if (index as i32) - ((self.columns * (word.len() - 1)) as i32) > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                let idx = index - (self.columns * i) + i - 1;
                let letter = self.letters[idx];
                println!(
                    "{i:}||{idx:}||{}:{}||{letter:}",
                    idx % self.columns,
                    (idx / self.columns)
                );
                chars.insert(i, letter);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn diagonal_bottom_left(&self, index: usize, word: &str) -> Option<String> {
        if index + self.columns * (word.len() - 1) - word.len() < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                let idx = index + (self.columns * i) - i;
                chars.insert(i, self.letters[idx]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    pub fn diagonal_bottom_right(&self, index: usize, word: &str) -> Option<String> {
        if index + self.columns * (word.len() - 1) + word.len() < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word.len());
            for i in 0..word.len() {
                chars.insert(i, self.letters[index + (self.columns * i) + i]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcd\nefgh", 2, vec![None, Some("ab"), Some("")])]
    fn test_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, c)| search.left(idx, wordl)).collect();
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    // #[case("MMMSXXMASS\nMSAMXMSMAA\nAMXSXMAAMM\nMSAMASMMMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    fn example(#[case] text: String, #[case] count: usize) {
        let search = WordSearch::from_string(text);
        assert_eq!(search.word_count("XMAS"), count);
    }
}
