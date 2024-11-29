pub fn part_a() {}

fn instruction_parser(formation: &mut LightFormation, instruction: &str) {
    let split: Vec<&str> = instruction.split(" ").collect();
    match split[0].to_lowercase().as_str() {
        "turn" => {
            let start = Coords::parse(split[2]);
            let finish = Coords::parse(split[4]);
            println!("{:#?}", start);
            println!("{:#?}", finish);
            match split[1].to_lowercase().as_str() {
                "on" => {
                    let mut y = start.y;
                    let mut start_index = start.x + (y * 1000);
                    let mut finish_index = finish.x + (y * 1000);
                    formation.turn_on(start_index, finish_index);

                    y += 1;
                    
                    while y <= finish.y {
                        start_index = start.x + (y * 1000);
                        finish_index = finish.x + (y * 1000);
                        formation.turn_on(start_index, finish_index);

                        y += 1;
                    }
                }
                "off" => {
                    let mut y = start.y;
                    let mut start_index = start.x + (y * 1000);
                    let mut finish_index = finish.x + (y * 1000);
                    formation.turn_off(start_index, finish_index);

                    y += 1;
                    
                    while y <= finish.y {
                        start_index = start.x + (y * 1000);
                        finish_index = finish.x + (y * 1000);
                        formation.turn_off(start_index, finish_index);

                        y += 1;
                    }
                }
                _ => panic!("Unhandled starting sentence! '{instruction}'"),
            }
        }
        "toggle" => {
            let start = Coords::parse(split[1]);
            let finish = Coords::parse(split[3]);
            let mut y = start.y;
            let mut start_index = start.x + (y * 1000);
            let mut finish_index = finish.x + (y * 1000);
            formation.toggle(start_index, finish_index);

            y += 1;
            
            while y <= finish.y {
                start_index = start.x + (y * 1000);
                finish_index = finish.x + (y * 1000);
                formation.toggle(start_index, finish_index);

                y += 1;
            }
        }
        _ => panic!("Unhandled starting sentence! '{instruction}'"),
    }
    println!("{:?}", split);
}

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}
impl Coords {
    pub fn parse(coords: &str) -> Self {
        let split: Vec<&str> = coords.split(",").collect();
        Self {
            x: split[0].parse::<usize>().unwrap(),
            y: split[1].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct LightFormation {
    lights: [bool; 1000000],
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [false; 1000000],
        }
    }

    pub fn number_of_lights_on(&self) -> usize {
        self.lights.iter().filter(|&&l| l == true).count()
    }

    pub fn toggle(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            self.lights[i] = !self.lights[i];
        }
    }

    pub fn turn_on(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            self.lights[i] = true;
        }
    }

    pub fn turn_off(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            self.lights[i] = false;
        }
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
        instruction_parser(&mut formation, instruction);
        assert_eq!(formation.number_of_lights_on(), lights_on);
    }

    #[rstest]
    #[case(vec!["turn on 0,0 through 999,999", "toggle 0,0 through 999,0", "turn off 499,499 through 500,500"], 4)]
    fn example_instructions(#[case] instructions: Vec<&str>, #[case] lights_on: usize) {
        let mut formation = LightFormation::new();
        for instruction in instructions.iter() {
            instruction_parser(&mut formation, instruction);
        }
        assert_eq!(formation.number_of_lights_on(), lights_on);
    }
}
