pub trait FormationTrait {
    fn number_of_lights_on(&self) -> usize;

    fn toggle(&mut self, start_index: usize, finish_index: usize);

    fn turn_on(&mut self, start_index: usize, finish_index: usize);

    fn turn_off(&mut self, start_index: usize, finish_index: usize);
}