fn main() {}

struct WordSearch {
    pub letters: Vec<Vec<char>>,
    pub x_size: usize,
    pub y_size: usize,
}

impl WordSearch {
    fn new() -> Self {
        Self {
            letters: Vec::new(),
            x_size: 0,
            y_size: 0,
        }
    }
    pub fn from_string(text: String) -> Self {
        let mut instance = Self::new();
        let mut x_size: usize = 0;
        let mut y_size: usize = 0;
        for row in text.lines().into_iter() {
            let value: Vec<char> = row.chars().collect();
            x_size = value.len();
            instance.letters.push(value);
            y_size += 1;
        }
        instance.x_size = x_size;
        instance.y_size = y_size;
        return instance;
    }

    pub fn word_count(&self, word: &str) -> usize {
        let mut count: usize = 0;
        let first_c: char = word.chars().next().unwrap();
        for (y, row) in self.letters.iter().enumerate() {
            for (idx, c) in row.iter().enumerate() {
                if *c == first_c {
                    if let Some(local_word) = self.left(idx, y, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.right(idx, y, self.x_size, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.top(idx, y, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.bottom(idx, y, self.y_size, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.diagonal_top_left(idx, y, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.diagonal_top_right(idx, y, self.x_size, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.diagonal_bottom_left(idx, y, self.y_size, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                    if let Some(local_word) = self.diagonal_bottom_right(idx, y, self.x_size, self.y_size, word.len()) {
                        if word == local_word.as_str() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn left(&self, x: usize, y: usize, word_size: usize) -> Option<String> {
        if x < word_size {
            return None;
        }
        Some(self.letters[y][x - word_size..x].iter().rev().collect())
    }

    fn right(&self, x: usize, y: usize, x_size: usize, word_size: usize) -> Option<String> {
        if x >= x_size - word_size {
            return None;
        }
        Some(self.letters[y][x..x + word_size].iter().collect())
    }

    fn top(&self, x: usize, y: usize, word_size: usize) -> Option<String> {
        if y < word_size {
            return None;
        }
        Some(self.letters[y - word_size..y][x].iter().rev().collect())
    }

    fn bottom(&self, x: usize, y: usize, y_size: usize, word_size: usize) -> Option<String> {
        if y >= y_size - word_size {
            return None;
        }
        Some(self.letters[y..y + word_size][x].iter().collect())
    }

    fn diagonal_top_left(&self, x: usize, y: usize, word_size: usize) -> Option<String> {
        if x < word_size && y < word_size {
            return None;
        }
        let mut res: String = String::from("");
        for idx in 0..word_size {
            res.push(self.letters[y - idx][x - idx]);
        }
        Some(res)
    }

    fn diagonal_top_right(&self, x: usize, y: usize, x_size: usize, word_size: usize) -> Option<String> {
        if x >= x_size - word_size && y < word_size {
            return None;
        }
        let mut res: String = String::from("");
        for idx in 0..word_size {
            res.push(self.letters[y - idx][x + idx]);
        }
        Some(res)
    }

    fn diagonal_bottom_left(&self, x: usize, y: usize, y_size: usize, word_size: usize) -> Option<String> {
        if x < word_size && y >= y_size - word_size {
            return None;
        }
        let mut res: String = String::from("");
        for idx in 0..word_size {
            res.push(self.letters[y + idx][x - idx]);
        }
        Some(res)
    }

    fn diagonal_bottom_right(&self, x: usize, y: usize, x_size: usize, y_size: usize, word_size: usize) -> Option<String> {
        if x >= x_size - word_size && y >= y_size - word_size {
            return None;
        }
        let mut res: String = String::from("");
        for idx in 0..word_size {
            res.push(self.letters[y + idx][x - idx]);
        }
        Some(res)
    }

    fn get_value(&self, x: usize, y: usize) -> char {
        self.letters[y][x]
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.left(x, y, wordl));
            }
        }
        println!("{words:?}");
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.right(x, y, search.x_size, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.top(x, y, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.bottom(x, y, search.y_size, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.diagonal_top_left(x, y, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.diagonal_top_right(x, y, search.x_size, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.diagonal_bottom_left(x, y, search.y_size, wordl));
            }
        }
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
        let mut words: Vec<Option<String>> = Vec::new();
        for (y, row) in search.letters.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                words.push(search.diagonal_bottom_right(x, y, search.x_size, search.y_size, wordl));
            }
        }
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
