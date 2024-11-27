use core::{enums::Part, file_reader::get_file_contents};

use crate::day_2::shared::Present;

pub fn part_b() {
    let mut total: i32 = 0;
    for line in get_file_contents(2015, 2, Part::A).lines().into_iter() {
        let [l, w, h]: [&str; 3] = line.split("x").collect::<Vec<&str>>().try_into().unwrap();
        let present = Present::from_strings(l, w, h);
        total += present.total_ribbon();
    }
    println!("Total ribbon: {total} square feet");
}

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
