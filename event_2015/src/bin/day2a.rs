use aoc_core::file_reader::get_file_contents;

use event_2015::shared::day2::Present;

pub fn main() {
    let mut total: i32 = 0;
    for line in get_file_contents(2015, 2).lines().into_iter() {
        let [l, w, h]: [&str; 3] = line.split("x").collect::<Vec<&str>>().try_into().unwrap();
        let present = Present::from_strings(l, w, h);
        total += present.surface_area();
    }
    println!("Total wrapping paper: {total} square feet");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 3, 4, 58)]
    #[case(1, 1, 10, 43)]
    fn examples(#[case] l: i32, #[case] w: i32, #[case] h: i32, #[case] area: i32) {
        assert_eq!(Present::new(l, w, h).surface_area(), area)
    }
}
