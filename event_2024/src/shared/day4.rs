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
                // println!("(x: {x_idx:}, y: {y_idx:})");
                let left_check: bool = x_idx >= wordl - 1;
                let right_check: bool = x_idx < row.len() - wordl + 1;
                let top_check: bool = y_idx >= wordl - 1;
                let bottom_check: bool = y_idx < self.words.len() - wordl;

                if *c == first_char {}
                // Left:
                if left_check {
                    // println!("LEFT");
                    // println!("[{} - {x_idx}]", x_idx + 1 - wordl);

                    let test_word = row[x_idx + 1 - wordl..=x_idx]
                        .iter()
                        .rev()
                        .collect::<String>();
                    // println!("LEFT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }
                // Right:
                if right_check {
                    // println!("RIGHT");
                    // println!("[{x_idx} - {}]", x_idx + wordl);

                    let test_word = row[x_idx..x_idx + wordl].iter().collect::<String>();
                    // println!("RIGHT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Top:
                if top_check {
                    // println!("TOP");
                    // println!("[{y_idx} -= {}]", y_idx + 1 - wordl);

                    let test_word = self.words[y_idx + 1 - wordl..=y_idx]
                        .iter()
                        .rev()
                        .map(|r| r[x_idx])
                        .collect::<String>();
                    // println!("TOP | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Top Left:
                if top_check && left_check {
                    // println!("TOP LEFT");
                    // println!("[{y_idx} -= {}]", y_idx + 1 - wordl);

                    let test_word = self.words[y_idx + 1 - wordl..=y_idx]
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(idx, r)| r[x_idx - idx])
                        .collect::<String>();
                    // println!("TOP LEFT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Top Right:
                if top_check && right_check {
                    // println!("TOP RIGHT");
                    // println!("[{y_idx} -= {}]", y_idx + 1 - wordl);

                    let test_word = self.words[y_idx + 1 - wordl..=y_idx]
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(idx, r)| r[x_idx + idx])
                        .collect::<String>();
                    // println!("TOP RIGHT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Bottom:
                if bottom_check {
                    // println!("BOTTOM");
                    // println!("[{} - {y_idx}]", y_idx..y_idx + wordl);

                    let test_word = self.words[y_idx..y_idx + wordl]
                        .iter()
                        .map(|r| r[x_idx])
                        .collect::<String>();
                    // println!("BOTTOM | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Bottom Left:
                if bottom_check && left_check {
                    // println!("BOTTOM LEFT");
                    // println!("[{} - {y_idx}]", y_idx..y_idx + wordl);

                    let test_word = self.words[y_idx..y_idx + wordl]
                        .iter()
                        .enumerate()
                        .map(|(idx, r)| r[x_idx - idx])
                        .collect::<String>();
                    // println!("BOTTOM LEFT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
                    }
                }

                // Bottom Right:
                if bottom_check && right_check {
                    // println!("BOTTOM RIGHT");
                    // println!("[{} - {y_idx}]", y_idx..y_idx + wordl);

                    let test_word = self.words[y_idx..y_idx + wordl]
                        .iter()
                        .enumerate()
                        .map(|(idx, r)| r[x_idx + idx])
                        .collect::<String>();
                    // println!("BOTTOM RIGHT | {test_word:}: {count}");
                    if word == test_word {
                        count += 1;
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
        search.display_search();
        assert_eq!(search.search(word), count);
    }
}
