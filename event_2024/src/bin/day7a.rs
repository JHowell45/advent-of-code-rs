use core::file_reader::get_file_contents;

use itertools::{repeat_n, Itertools};

fn main() {
    println!("Total calibration result: {}", sum_of_valid_results(get_file_contents(2024, 7).as_str()));
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn sum_of_valid_results(text: &str) -> i64 {
    text.lines()
        .map(|equation| {
            let (result, numbers) = parse_input(equation);
            // println!("{result:} : {numbers:?}");
            if validate_equation(result, numbers) {
                return result;
            }
            return 0;
        })
        .sum()
}

fn parse_input(input: &str) -> (i64, Vec<i64>) {
    let split: Vec<&str> = input.split(": ").collect();
    (
        split[0].parse::<i64>().unwrap(),
        split[1]
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect(),
    )
}

fn validate_equation(result: i64, numbers: Vec<i64>) -> bool {
    // println!("{result:}, {numbers:?} || {}", numbers.len() - 1);
    for operators in repeat_n(
        vec![Operator::Add, Operator::Multiply].iter(),
        numbers.len() - 1,
    )
    .multi_cartesian_product()
    {
        // println!(
        //     "{result:}, {numbers:?} || {} || {operators:?}",
        //     numbers.len() - 1
        // );
        if validate_equation_ops(result, &numbers, &operators) {
            return true;
        }
    }
    return false;
}

fn validate_equation_ops(result: i64, numbers: &Vec<i64>, operators: &Vec<&Operator>) -> bool {
    let mut total: i64 = numbers[0];
    for idx in 1..numbers.len() {
        let v = numbers[idx];
        let op = operators[idx - 1];
        match op {
            Operator::Add => total += v,
            Operator::Multiply => total *= v,
        }
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
    #[case(190, vec![10, 19], vec![&Operator::Add], false)]
    #[case(190, vec![10, 19], vec![&Operator::Multiply], true)]
    fn test_validate_equation_ops(
        #[case] result: i32,
        #[case] numbers: Vec<i32>,
        #[case] ops: Vec<&Operator>,
        #[case] expected: bool,
    ) {
        assert_eq!(validate_equation_ops(result, &numbers, &ops), expected);
    }

    #[rstest]
    #[case(190, vec![10, 19], true)]
    #[case(3267, vec![81, 40, 27], true)]
    #[case(83, vec![17, 5], false)]
    #[case(156, vec![15, 6], false)]
    #[case(7290, vec![6, 8, 6, 15], false)]
    #[case(161011, vec![16, 10, 13], false)]
    #[case(192, vec![17, 8, 14], false)]
    #[case(21037, vec![9, 7, 18, 13], false)]
    #[case(292, vec![11, 6, 16, 20], true)]
    fn test_validate_equation(
        #[case] result: i32,
        #[case] numbers: Vec<i32>,
        #[case] expected: bool,
    ) {
        assert_eq!(validate_equation(result, numbers), expected);
    }

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
