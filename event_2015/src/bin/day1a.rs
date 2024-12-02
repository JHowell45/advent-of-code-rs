use core::{file_reader::get_file_contents};

pub fn main() {
    let contents = get_file_contents(2015, 1);
    println!("Floor: {}", calculate_floor(&contents));
}

fn calculate_floor(directions: &str) -> i32 {
    let mut floor = 0;
    for instruction in directions.chars().into_iter() {
        floor += match instruction {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }
    return floor;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn examples(#[case] instructions: &str, #[case] floor: i32) {
        assert_eq!(calculate_floor(instructions), floor)
    }
}
