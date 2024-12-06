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
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| self.letters[index - (self.columns * i)])
                .collect();
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn bottom(&self, index: usize, word_length: usize) -> Option<String> {
        if index + self.columns * (word_length - 1) < self.letters.len() {
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| self.letters[index + (self.columns * i)])
                .collect();
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_top_left(&self, index: usize, word_length: usize) -> Option<String> {
        if index > (self.columns * (word_length - 1)) && index % self.columns > word_length - 2 {
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| self.letters[index - (self.columns * i) - i])
                .collect();
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_top_right(&self, index: usize, word_length: usize) -> Option<String> {
        if index > (self.columns * (word_length - 1)) - 1
            && index % self.columns < self.columns - word_length + 1
        {
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| {
                    let idx = index - (self.columns * i) + i;
                    // println!(
                    //     "{} | {index:} | {idx} | {} | {}",
                    //     self.columns, index % self.columns, self.letters[idx]
                    // );
                    self.letters[idx]
                })
                .collect();
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_bottom_left(&self, index: usize, word_length: usize) -> Option<String> {
        if index < self.letters.len() - (self.columns * (word_length - 1))
            && index % self.columns > word_length - 2
        {
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| {
                    let idx = index + (self.columns * i) - i;
                    // println!(
                    //     "{} | {index:} | {idx} | {}",
                    //     self.columns, self.letters[idx]
                    // );
                    self.letters[idx]
                })
                .collect();
            let local_word: String = chars.iter().collect();
            return Some(local_word);
        }
        return None;
    }

    fn diagonal_bottom_right(&self, index: usize, word_length: usize) -> Option<String> {
        if index < self.letters.len() - (self.columns * (word_length - 1))
            && index % self.columns < self.columns - word_length + 1
        {
            let chars: Vec<char> = (0..word_length)
                .into_iter()
                .map(|i| {
                    let idx = index + (self.columns * i) + i;
                    // println!(
                    //     "{} | {index:} | {idx} | {}",
                    //     self.columns, self.letters[idx]
                    // );
                    self.letters[idx]
                })
                .collect();
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
    #[case("abcd\nefgh", 2, vec![
        None, Some(String::from("ba")), Some(String::from("cb")),  Some(String::from("dc")),
        None, Some(String::from("fe")), Some(String::from("gf")), Some(String::from("hg"))
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        None, None, None, Some(String::from("dcba")), Some(String::from("edcb")), Some(String::from("fedc")), Some(String::from("gfed")), Some(String::from("hgfe")),
        None, None, None, Some(String::from("lkji")), Some(String::from("mlkj")), Some(String::from("nmlk")), Some(String::from("onml")), Some(String::from("ponm")),
        None, None, None, Some(String::from("4321")), Some(String::from("5432")), Some(String::from("6543")), Some(String::from("7654")), Some(String::from("8765")),
        None, None, None, Some(String::from("dcba")), Some(String::from("edcb")), Some(String::from("fedc")), Some(String::from("gfed")), Some(String::from("hgfe"))
    ])]
    fn test_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.left(idx, wordl))
            .collect();
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![
        Some(String::from("ab")), Some(String::from("bc")), Some(String::from("cd")), None,
        Some(String::from("ef")), Some(String::from("fg")), Some(String::from("gh")), None
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        Some(String::from("abcd")), Some(String::from("bcde")), Some(String::from("cdef")), Some(String::from("defg")), Some(String::from("efgh")), None, None, None,
        Some(String::from("ijkl")), Some(String::from("jklm")), Some(String::from("klmn")), Some(String::from("lmno")), Some(String::from("mnop")), None, None, None,
        Some(String::from("1234")), Some(String::from("2345")), Some(String::from("3456")), Some(String::from("4567")), Some(String::from("5678")), None, None, None,
        Some(String::from("abcd")), Some(String::from("bcde")), Some(String::from("cdef")), Some(String::from("defg")), Some(String::from("efgh")), None, None, None
    ])]
    fn test_right(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.right(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![
        None, None, None, None,
        Some(String::from("ea")), Some(String::from("fb")), Some(String::from("gc")), Some(String::from("hd"))
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        Some(String::from("a1ia")), Some(String::from("b2jb")), Some(String::from("c3kc")), Some(String::from("d4ld")), Some(String::from("e5me")), Some(String::from("f6nf")), Some(String::from("g7og")), Some(String::from("h8ph")),
    ])]
    fn test_top(#[case] text: String, #[case] wordl: usize, #[case] expected: Vec<Option<String>>) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.top(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh", 2, vec![Some(String::from("ae")), Some(String::from("bf")), Some(String::from("cg")), Some(String::from("dh")), None, None, None, None])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        Some(String::from("ai1a")), Some(String::from("bj2b")), Some(String::from("ck3c")), Some(String::from("dl4d")), Some(String::from("em5e")), Some(String::from("fn6f")), Some(String::from("go7g")), Some(String::from("hp8h")),
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None
    ])]
    fn test_bottom(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.bottom(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh\nijkl\nmnop", 2, vec![
        None, None, None, None,
        None, Some(String::from("fa")), Some(String::from("gb")), Some(String::from("hc")), 
        None, Some(String::from("je")), Some(String::from("kf")), Some(String::from("lg")),
        None, Some(String::from("ni")), Some(String::from("oj")), Some(String::from("pk"))
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, Some(String::from("d3ja")), Some(String::from("e4kb")), Some(String::from("f5lc")), Some(String::from("g6md")), Some(String::from("h7ne"))
    ])]
    fn test_diagonal_top_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.diagonal_top_left(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh\nijkl\nmnop", 2, vec![
        None, None, None, None,
        Some(String::from("eb")), Some(String::from("fc")), Some(String::from("gd")), None,
        Some(String::from("if")), Some(String::from("jg")), Some(String::from("kh")), None,
        Some(String::from("mj")), Some(String::from("nk")), Some(String::from("ol")), None,
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        Some(String::from("a2kd")), Some(String::from("b3le")), Some(String::from("c4mf")), Some(String::from("d5ng")), Some(String::from("e6oh")), None, None, None
    ])]
    fn test_diagonal_top_right(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.diagonal_top_right(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh\nijkl\nmnop", 2, vec![
        None, Some(String::from("be")), Some(String::from("cf")), Some(String::from("dg")),
        None, Some(String::from("fi")), Some(String::from("gj")), Some(String::from("hk")),
        None, Some(String::from("jm")), Some(String::from("kn")), Some(String::from("lo")),
        None, None, None, None
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        None, None, None, Some(String::from("dk2a")), Some(String::from("el3b")), Some(String::from("fm4c")), Some(String::from("gn5d")), Some(String::from("ho6e")),
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None
    ])]
    fn test_diagonal_bottom_left(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.diagonal_bottom_left(idx, wordl))
            .collect();
        println!("{words:?}");
        assert_eq!(words, expected);
    }

    #[rstest]
    #[case("abcd\nefgh\nijkl\nmnop", 2, vec![
        Some(String::from("af")), Some(String::from("bg")), Some(String::from("ch")), None,
        Some(String::from("ej")), Some(String::from("fk")), Some(String::from("gl")), None,
        Some(String::from("in")), Some(String::from("jo")), Some(String::from("kp")), None,
        None, None, None, None
    ])]
    #[case("abcdefgh\nijklmnop\n12345678\nabcdefgh", 4, vec![
        Some(String::from("aj3d")), Some(String::from("bk4e")), Some(String::from("cl5f")), Some(String::from("dm6g")), Some(String::from("en7h")), None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None
    ])]
    fn test_diagonal_bottom_right(
        #[case] text: String,
        #[case] wordl: usize,
        #[case] expected: Vec<Option<String>>,
    ) {
        let search = WordSearch::from_string(text);
        let words: Vec<Option<String>> = search
            .letters
            .iter()
            .enumerate()
            .map(|(idx, _c)| search.diagonal_bottom_right(idx, wordl))
            .collect();
        println!("{words:?}");
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
