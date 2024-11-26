use clap::Parser;
use core::enums::Part;

#[derive(Parser)]
struct Cli {
    year: i32,
    day: i32,
    part: Part,
}

impl Cli {
    pub fn run(&self) {
        match self.year {
            2015 => match self.day {
                1 => match self.part {
                    Part::A => event_2015::day_1::part_a::part_a(),
                    Part::B => event_2015::day_1::part_b::part_b(),
                },
                2 => match self.part {
                    Part::A => event_2015::day_2::part_a::part_a(),
                    Part::B => event_2015::day_2::part_b::part_b(),
                },
                3 => match self.part {
                    Part::A => event_2015::day_3::part_a::part_a(),
                    Part::B => event_2015::day_3::part_b::part_b(),
                },
                _ => panic!("Invalid day!"),
            },
            _ => panic!("Invalid year!"),
        }
    }
}

fn main() {
    Cli::parse().run();
}
