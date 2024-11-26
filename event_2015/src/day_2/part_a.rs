use core::{enums::Part, file_reader::get_file_contents};

#[derive(Debug)]
pub struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    pub fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    pub fn from_strings(length: &str, width: &str, height: &str) -> Self {
        Self {
            length: length.parse::<i32>().unwrap(),
            width: width.parse::<i32>().unwrap(),
            height: height.parse::<i32>().unwrap(),
        }
    }

    pub fn surface_area(&self) -> i32 {
        (2 * self.length_width_area())
            + (2 * self.width_height_area())
            + (2 * self.height_length_area()) + self.slack()
    }

    fn slack(&self) -> i32 {
        *vec![
            self.length_width_area(),
            self.width_height_area(),
            self.height_length_area(),
        ]
        .iter()
        .min().unwrap()
    }

    fn length_width_area(&self) -> i32 {
        self.length * self.width
    }

    fn width_height_area(&self) -> i32 {
        self.width * self.height
    }

    fn height_length_area(&self) -> i32 {
        self.height * self.length
    }
}

pub fn part_a() {
    let mut total: i32 = 0;    
    for line in get_file_contents(2015, 2, Part::A).lines().into_iter() {
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
