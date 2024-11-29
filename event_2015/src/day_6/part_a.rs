pub fn part_a() {}

fn instruction_parser(instruction: &str) {}

struct LightFormation {
    lights: [bool; 1000000]
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [false; 1000000]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn examples() {

    }
}