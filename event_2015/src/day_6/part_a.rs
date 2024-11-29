pub fn part_a() {}

pub fn instruction_parser(formation: &LightFormation, instruction: &str) {}

struct LightFormation {
    lights: [bool; 1000000]
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [false; 1000000]
        }
    }

    pub fn number_of_lights_on(&self) -> usize {}

    pub fn toggle(&mut self) {}

    pub fn turn_on(&mut self) {}

    pub fn turn_off(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1000000)]
    #[case("toggle 0,0 through 999,0", 1000)]
    fn example_instruction(#[case] instruction: &str, #[case] lights_on: usize) {
        let mut formation = LightFormation::new();
        instruction_parser(&formation, instruction);
        assert_eq!(formation.number_of_lights_on(), lights_on);
    }

    #[rstest]
    #[case(vec!["turn on 0,0 through 999,999", "toggle 0,0 through 999,0", "turn off 499,499 through 500,500"], 4)]
    fn example_instructions(#[case] instructions: Vec<&str>, #[case] lights_on: usize) {
        let mut formation = LightFormation::new();
        for instruction in instructions.iter() {
            instruction_parser(&formation, instruction);
        }
        assert_eq!(formation.number_of_lights_on(), lights_on);
    }
}