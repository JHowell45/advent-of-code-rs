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
                println!("({x_idx:}, {y_idx:})");
                if *c == first_char {
                    // Left:
                    if x_idx >= wordl - 1 {
                        // println!("LEFT");
                        let test_word = row[x_idx + 1 - wordl..x_idx].iter().rev().collect::<String>();
                        println!("LEFT | {test_word:}: {count}");
                        if word == test_word {
                            count += 1;
                        }
                    }
                    // Right:
                    if x_idx < row.len() - wordl {
                        // println!("RIGHT");
                        let test_word = row[x_idx..x_idx + wordl].iter().collect::<String>();
                        println!("RIGHT | {test_word:}: {count}");
                        if word == test_word {
                            count += 1;
                        }
                    }

                    // Top:
                    if y_idx >= wordl - 1 {
                        // println!("TOP");
                        let mut test_chars: Vec<char> = Vec::new();
                        for idx in (y_idx + 1 - wordl..y_idx).rev() {
                            test_chars.push(self.words[idx][x_idx]);
                        }
                        let test_word = test_chars.iter().collect::<String>();
                        println!("TOP | {test_word:}: {count}");
                        if word == test_word {
                            count += 1;
                        }
                    }

                    // Bottom:
                    if y_idx < self.words.len() {
                        // println!("BOTTOM");
                        let mut test_chars: Vec<char> = Vec::new();
                        for idx in (y_idx..y_idx + wordl) {
                            test_chars.push(self.words[idx][x_idx]);
                        }
                        let test_word = test_chars.iter().collect::<String>();
                        println!("BOTTOM | {test_word:}: {count}");
                        if word == test_word {
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
