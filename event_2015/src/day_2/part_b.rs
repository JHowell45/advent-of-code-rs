use crate::day_2::shared::Present;

pub fn part_b() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 3, 4, 34)]
    #[case(1, 1, 10, 14)]
    fn examples(#[case] l: i32, #[case] w: i32, #[case] h: i32, #[case] ribbon: i32) {
        assert_eq!(Present::new(l, w, h).total_ribbon(), ribbon)
    }
}