fn main() {}

enum Operator {
    Plus,
    Multiply
}

fn parse_input(input: &str) -> (i32, Vec<i32>) {
    let split: Vec<&str> = input.split(": ").collect();
    (
        split[0].parse::<i32>().unwrap(),
        split[1].split(" ").map(|n| n.parse::<i32>().unwrap()).collect()
    )
}

fn evaluate_function(result: i32, numbers: Vec<i32>, ops: Vec<i32>) -> bool {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}