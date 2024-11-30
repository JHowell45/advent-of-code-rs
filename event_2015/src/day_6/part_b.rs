use super::shared::FormationTrait;

pub fn part_b() {}

pub struct LightFormation {
    lights: [i32; 1000000]
}

impl LightFormation {
    pub fn new() -> Self {
        Self {
            lights: [0; 1000000]
        }
    }
}

impl FormationTrait for LightFormation {
    fn number_of_lights_on(&self) -> usize {
        self.lights.iter().sum()
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