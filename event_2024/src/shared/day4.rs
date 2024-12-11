pub struct WordSearch {
    words: Vec<Vec<char>>,
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
        Self { words: word_search }
    }

    pub fn search(&self, word: &str) -> usize {
        let mut count: usize = 0;
        let first_char = word.chars().next().unwrap();
        let wordl: usize = word.len();
        for (y_idx, row) in self.words.iter().enumerate() {
            for (x_idx, c) in row.iter().enumerate() {
                if *c == first_char {
                    // Left:
                    if x_idx >= wordl - 1 {
                        if word == row[x_idx - wordl..x_idx].iter().rev().collect::<String>() {
                            count += 1;
                        }
                    }
                    // Right:
                    if x_idx < row.len() - wordl {
                        if word == row[x_idx..x_idx + wordl].iter().collect::<String>() {
                            count += 1;
                        }
                    }


                    // Top:
                    if y_idx >= wordl - 1 {
                        let mut test_word: Vec<char> = Vec::new();
                        for idx in (y_idx - wordl..y_idx).rev() {
                            test_word.push(self.words[idx][x_idx]);
                        }
                        if word == test_word.iter().collect::<String>() {
                            count += 1;
                        }
                    }
                }
            }
        }
        return count;
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
    #[case(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        "XMAS",
        18
    )]
    fn example(#[case] input: &str, #[case] word: &str, #[case] count: usize) {
        let search = WordSearch::from_string(input);
        assert_eq!(search.search(word), count);
    }
}
