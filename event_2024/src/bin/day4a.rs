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
            if let Some(local_word) = self.right(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.top(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_top_left(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_top_right(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.bottom(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_bottom_left(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            if let Some(local_word) = self.diagonal_bottom_right(index, word.len()) {
                if word == local_word.as_str() {
                    count += 1;
                }
            }
        }
        println!("{:?}", self);
        return count;
    }

    fn left(&self, index: usize, word_length: usize) -> Option<String> {
        if index % self.columns >= word_length - 1 {
            let idx = index - (word_length - 1);
            let local_word: String = self.letters[idx..=index].iter().rev().collect();
            return Some(local_word);
        }
        return None;
    }

    fn right(&self, index: usize, word_length: usize) -> Option<String> {
        if index % self.columns < self.columns - word_length + 1 {
            let local_word: String = self.letters[index..index + word_length].iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn top(&self, index: usize, word_length: usize) -> Option<String> {
        if (index as i32) - (self.columns * (word_length - 1)) as i32 + 1 > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
                chars.insert(i, self.letters[index - (self.columns * i)]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn bottom(&self, index: usize, word_length: usize) -> Option<String> {
        if index + self.columns * (word_length - 1) < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
                chars.insert(i, self.letters[index + (self.columns * i)]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_top_left(&self, index: usize, word_length: usize) -> Option<String> {
        if (index as i32) - (self.columns * (word_length - 1)) as i32 - (word_length as i32) - 2 > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
                chars.insert(i, self.letters[index - (self.columns * i) - i]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_top_right(&self, index: usize, word_length: usize) -> Option<String> {
        if (index as i32) - ((self.columns * (word_length - 1)) as i32) > 0 {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
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

    fn diagonal_bottom_left(&self, index: usize, word_length: usize) -> Option<String> {
        if index + self.columns * (word_length - 1) - word_length < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
                let idx = index + (self.columns * i) - i;
                chars.insert(i, self.letters[idx]);
            }
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_bottom_right(&self, index: usize, word_length: usize) -> Option<String> {
        if index + self.columns * (word_length - 1) + word_length < self.letters.len() {
            let mut chars: Vec<char> = Vec::with_capacity(word_length);
            for i in 0..word_length {
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
    #[case("abcd\nefgh", 2, vec![None, Some(String::from("ba")), Some(String::from("cb")),  Some(String::from("dc")), None, Some(String::from("fe")), Some(String::from("gf")), Some(String::from("hg"))])]
    fn test_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, _c)| search.left(idx, wordl)).collect();
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![Some(String::from("ab")), Some(String::from("bc")), Some(String::from("cd")), None, Some(String::from("ef")), Some(String::from("fg")), Some(String::from("gh")), None])]
    fn test_right(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, _c)| search.right(idx, wordl)).collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![None, None, None, None, Some(String::from("ea")), Some(String::from("fb")), Some(String::from("gc")), Some(String::from("hd"))])]
    fn test_top(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, _c)| search.top(idx, wordl)).collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![Some(String::from("ae")), Some(String::from("bf")), Some(String::from("cg")), Some(String::from("dh")), None, None, None, None])]
    fn test_bottom(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, _c)| search.bottom(idx, wordl)).collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh\nijkl\nmnop", 2, vec![
        None, None, None, None, 
        None, Some(String::from("fa")), Some(String::from("gb")), Some(String::from("hc")), 
        None, Some(String::from("je")), Some(String::from("kf")), Some(String::from("lh")),
        None, Some(String::from("ni")), Some(String::from("oj")), Some(String::from("pk"))
    ])]
    fn test_diagonal_top_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search.letters.iter().enumerate().map(|(idx, _c)| search.diagonal_top_left(idx, wordl)).collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    // #[rstest]
    // #[case("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    // // #[case("MMMSXXMASS\nMSAMXMSMAA\nAMXSXMAAMM\nMSAMASMMMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    // fn example(#[case] text: String, #[case] count: usize) {
    //     let search = WordSearch::from_string(text);
    //     assert_eq!(search.word_count("XMAS"), count);
    // }
}