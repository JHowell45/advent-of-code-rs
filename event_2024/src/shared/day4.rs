pub struct WordSearch {
    words: Vec<Vec<char>>
}

impl WordSearch {
    pub fn from_string(text: &str) -> Self {
        let mut word_search: Vec<Vec<char>> = Vec::new();
        for row in text.split("\n").into_iter() {
            let mut row_search: Vec<char> = Vec::new();
            for c in row.chars().into_iter() {
                row_search.push(c);
            }
            word_search.push(row_search);
        }
        Self {
            words: word_search
        }
    }

    pub fn search(&self, word: &str) -> usize {
0
    }
    pub fn display_search(&self) {
        for row in self.words.iter() {
            for c in row {
                print!("{c:}");
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
", "XMAS", 18)]
    fn example(#[case] input: &str, #[case] word: &str, #[case] count: usize) {
        let search = WordSearch::from_string(input);
        assert_eq!(search.search(word), count);
    }
}