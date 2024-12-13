use core::file_reader::get_file_contents;

use event_2024::shared::day13::ClawMachine;

fn main() {
    println!(
        "Total Tokens: {}",
        total_tokens_offset(get_file_contents(2024, 13).as_str(), 10000000000000)
    );
}

fn total_tokens_offset(text: &str, offset: usize) -> usize {
    text.split("\n\n")
        .into_iter()
        .map(|g| {
            let mut machine: ClawMachine = ClawMachine::from_string(g);
            return match machine.least_tokens_offset(offset) {
                Some(tokens) => tokens,
                None => 0,
            };
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_2024::shared::day13::ClawMachine;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        480
    )]
    fn test_total_tokens_offset(#[case] input: &str, #[case] tokens: usize) {
        assert_eq!(total_tokens_offset(input, 10000000000000), tokens);
    }

    #[rstest]
    #[case(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        Some(280)
    )]
    #[case(
        "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176",
        None
    )]
    #[case(
        "Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450",
        Some(200)
    )]
    #[case(
        "Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        None
    )]
    fn example(#[case] input: &str, #[case] tokens: Option<usize>) {
        let machine: ClawMachine = ClawMachine::from_string(input);
        // println!("{input:}");
        // println!("{machine:?}");
        assert_eq!(machine.least_tokens(), tokens);
    }
}
