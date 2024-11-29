pub fn part_a() {}

pub fn instruction_parser(formation: &LightFormation, instruction: &str) {
    let split: Vec<&str> = instruction.split(" ").collect();
    match split[0].to_lowercase().as_str() {
        "turn" => {
            match split[1].to_lowercase().as_str() {
                "on" => {

                },
                "off" => {

                },
                _ => panic!("Unhandled starting sentence! '{instruction}'")
            }
        },
        "toggle" => {
            let start = Coords::parse(split[1]);
            let finish = Coords::parse(split[3]);
        }
        _ => panic!("Unhandled starting sentence! '{instruction}'")
    }
    println!("{:?}", split);
}

struct Coords {

}
impl Coords {
    pub fn parse(coords: &str) -> Self {
        Self {}
    }
}

struct LightFormation {
    lights: [bool; 1000000]
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [false; 1000000]
        }
    }

    pub fn number_of_lights_on(&self) -> usize {
        self.lights.iter().filter(|l| **l == true).count()
    }

    pub fn toggle(&mut self, start_index: usize, finish_index: usize) {
        let _ = self.lights[start_index..finish_index].iter_mut().map(|l| *l = !(*l));
    }

    pub fn turn_on(&mut self, start_index: usize, finish_index: usize) {
        let _ = self.lights[start_index..finish_index].iter_mut().map(|l| *l = true);
    }

    pub fn turn_off(&mut self, start_index: usize, finish_index: usize) {
        let _ = self.lights[start_index..finish_index].iter_mut().map(|l| *l = false);
    }
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