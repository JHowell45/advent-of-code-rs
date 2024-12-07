use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Concat,
}

pub fn sum_of_valid_results(text: &str) -> i64 {
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

pub fn parse_input(input: &str) -> (i64, Vec<i64>) {
    let split: Vec<&str> = input.split(": ").collect();
    (
        split[0].parse::<i64>().unwrap(),
        split[1]
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect(),
    )
}

pub fn validate_equation(result: i64, numbers: Vec<i64>) -> bool {
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

pub fn validate_equation_ops(result: i64, numbers: &Vec<i64>, operators: &Vec<&Operator>) -> bool {
    let mut total: i64 = numbers[0];
    for idx in 1..numbers.len() {
        let v = numbers[idx];
        let op = operators[idx - 1];
        match op {
            Operator::Add => total += v,
            Operator::Multiply => total *= v,
            Operator::Concat => {
                let mut a: String = total.to_string();
                a.push_str(v.to_string().as_str());
                total = a.parse::<i64>().unwrap();
            }
        }
        if total > result {
            return false;
        }
    }
    return total == result;
}
