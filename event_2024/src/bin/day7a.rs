use itertools::Itertools;

fn main() {}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn sum_of_valid_results(text: &str) -> i32 {
    text.lines()
        .map(|equation| {
            let (result, numbers) = parse_input(equation);
            println!("{result:} : {numbers:?}");
            if validate_equation(result, numbers) {
                return result;
            }
            return 0;
        })
        .sum()
}

fn parse_input(input: &str) -> (i32, Vec<i32>) {
    let split: Vec<&str> = input.split(": ").collect();
    (
        split[0].parse::<i32>().unwrap(),
        split[1]
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect(),
    )
}

fn validate_equation(result: i32, numbers: Vec<i32>) -> bool {
    for operators in [Operator::Add, Operator::Multiply]
        .iter()
        .combinations(numbers.len() - 1)
    {
        println!("ops: {operators:?}");
        if validate_equation_ops(result, &numbers, &operators) {
            return true;
        }
    }
    return false;
}

fn validate_equation_ops(result: i32, numbers: &Vec<i32>, operators: &Vec<&Operator>) -> bool {
    let mut total = numbers[0];
    println!("{operators:?}");
    for idx in 1..numbers.len() {
        let v = numbers[idx];
        let op = operators[idx - 1];

        print!("{total:} {op:?} {v:}");
        match op {
            Operator::Add => total += v,
            Operator::Multiply => total *= v,
        }
        println!(" = {total:} ({result:})");
        if total > result {
            return false;
        }
    }
    return total == result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        3749
    )]
    fn example(#[case] data: &str, #[case] expected: i32) {
        assert_eq!(sum_of_valid_results(data), expected);
    }
}
