use core::file_reader::get_file_contents;

pub fn main() {
    let contents = get_file_contents(2015, 1);
    println!("Floor: {}", first_floor_in_basement(&contents));
}

fn first_floor_in_basement(directions: &str) -> i32 {
    let mut floor: i32 = 0;
    for (index, instruction) in directions.chars().enumerate() {
        floor += match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            return index as i32 + 1;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn examples(#[case] instructions: &str, #[case] floor: i32) {
        assert_eq!(first_floor_in_basement(instructions), floor)
    }
}
