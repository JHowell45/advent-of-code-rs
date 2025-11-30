use aoc_core::file_reader::get_file_contents;

use event_2024::shared::day18::MemorySpace;

fn main() {
    let mut space = MemorySpace::from_string(70, get_file_contents(2024, 18).as_str());
    println!("Least number of steps: {}", space.least_steps(1024));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}
