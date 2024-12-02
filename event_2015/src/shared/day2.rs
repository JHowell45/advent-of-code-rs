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
            + (2 * self.height_length_area())
            + self.slack()
    }

    pub fn total_ribbon(&self) -> i32 {
        let mut lengths = vec![self.length, self.width, self.height];
        lengths.sort();
        (2 * lengths[0]) + (2 * lengths[1]) + (self.length * self.width * self.height)
    }

    fn slack(&self) -> i32 {
        *vec![
            self.length_width_area(),
            self.width_height_area(),
            self.height_length_area(),
        ]
        .iter()
        .min()
        .unwrap()
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
