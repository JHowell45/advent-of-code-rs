#[derive(Debug)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
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

pub trait FormationTrait {
    fn number_of_lights_on(&self) -> usize;

    fn toggle(&mut self, start_index: usize, finish_index: usize);

    fn turn_on(&mut self, start_index: usize, finish_index: usize);

    fn turn_off(&mut self, start_index: usize, finish_index: usize);
}
