use core::{enums::Part, file_reader::get_file_contents};

use super::shared::{Coords, FormationTrait};

pub fn part_b() {
    let mut formation = LightFormation::new();
    for instruction in get_file_contents(2015, 6, Part::A).lines().into_iter() {
        instruction_parser(&mut formation, instruction);
    }
    println!(
        "Total light brightness: {}",
        formation.number_of_lights_on()
    );
}

fn instruction_parser(formation: &mut LightFormation, instruction: &str) {
    let split: Vec<&str> = instruction.split(" ").collect();
    match split[0].to_lowercase().as_str() {
        "turn" => {
            let start = Coords::parse(split[2]);
            let finish = Coords::parse(split[4]);
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
}

pub struct LightFormation {
    lights: [i32; 1000000],
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [0; 1000000],
        }
    }
}

impl FormationTrait for LightFormation {
    fn number_of_lights_on(&self) -> usize {
        usize::try_from(self.lights.iter().sum::<i32>()).unwrap()
    }

    fn toggle(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            self.lights[i] += 2;
        }
    }

    fn turn_on(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            self.lights[i] += 1;
        }
    }

    fn turn_off(&mut self, start_index: usize, finish_index: usize) {
        for i in start_index..=finish_index {
            if self.lights[i] > 0 {
                self.lights[i] -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // #[rstest]
    // #[case("turn on 0,0 through 0,0", 1)]
    // #[case("toggle 0,0 through 999,999", 2000000)]
    // fn examples(#[case] instruction: &str, #[case] brightness: usize) {
    //     let mut formation = LightFormation::new();
    //     instruction_parser(&mut formation, instruction);
    //     assert_eq!(formation.number_of_lights_on(), brightness);
    // }
}
