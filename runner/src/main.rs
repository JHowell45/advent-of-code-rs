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
                4 => match self.part {
                    Part::A => event_2015::day_4::part_a::part_a(),
                    Part::B => event_2015::day_4::part_b::part_b(),
                },
                5 => match self.part {
                    Part::A => event_2015::day_5::part_a::part_a(),
                    Part::B => event_2015::day_5::part_b::part_b(),
                },
                6 => match self.part {
                    Part::A => event_2015::day_6::part_a::part_a(),
                    Part::B => event_2015::day_6::part_b::part_b(),
                },
                _ => panic!("Invalid day!"),
            },
            2024 => match self.day {
                1 => match self.part {
                    Part::A => event_2024::day_1::part_a::part_a(),
                    Part::B => event_2024::day_1::part_b::part_b(),
                },
                2 => match self.part {
                    Part::A => event_2024::day_2::part_a::part_a(),
                    Part::B => event_2024::day_2::part_b::part_b(),
                },
                // 3 => match self.part {
                //     Part::A => event_2024::day_3::part_a::part_a(),
                //     Part::B => event_2024::day_3::part_b::part_b(),
                // },
                // 4 => match self.part {
                //     Part::A => event_2024::day_4::part_a::part_a(),
                //     Part::B => event_2024::day_4::part_b::part_b(),
                // },
                // 5 => match self.part {
                //     Part::A => event_2024::day_5::part_a::part_a(),
                //     Part::B => event_2024::day_5::part_b::part_b(),
                // },
                // 6 => match self.part {
                //     Part::A => event_2024::day_6::part_a::part_a(),
                //     Part::B => event_2024::day_6::part_b::part_b(),
                // },
                // 6 => match self.part {
                //     Part::A => event_2024::day_6::part_a::part_a(),
                //     Part::B => event_2024::day_6::part_b::part_b(),
                // },
                // 7 => match self.part {
                //     Part::A => event_2024::day_7::part_a::part_a(),
                //     Part::B => event_2024::day_7::part_b::part_b(),
                // },
                // 8 => match self.part {
                //     Part::A => event_2024::day_8::part_a::part_a(),
                //     Part::B => event_2024::day_8::part_b::part_b(),
                // },
                // 9 => match self.part {
                //     Part::A => event_2024::day_9::part_a::part_a(),
                //     Part::B => event_2024::day_9::part_b::part_b(),
                // },
                // 10 => match self.part {
                //     Part::A => event_2024::day_10::part_a::part_a(),
                //     Part::B => event_2024::day_10::part_b::part_b(),
                // },
                // 11 => match self.part {
                //     Part::A => event_2024::day_11::part_a::part_a(),
                //     Part::B => event_2024::day_11::part_b::part_b(),
                // },
                // 12 => match self.part {
                //     Part::A => event_2024::day_12::part_a::part_a(),
                //     Part::B => event_2024::day_12::part_b::part_b(),
                // },
                // 13 => match self.part {
                //     Part::A => event_2024::day_13::part_a::part_a(),
                //     Part::B => event_2024::day_13::part_b::part_b(),
                // },
                // 14 => match self.part {
                //     Part::A => event_2024::day_14::part_a::part_a(),
                //     Part::B => event_2024::day_14::part_b::part_b(),
                // },
                // 15 => match self.part {
                //     Part::A => event_2024::day_15::part_a::part_a(),
                //     Part::B => event_2024::day_15::part_b::part_b(),
                // },
                // 16 => match self.part {
                //     Part::A => event_2024::day_16::part_a::part_a(),
                //     Part::B => event_2024::day_16::part_b::part_b(),
                // },
                // 17 => match self.part {
                //     Part::A => event_2024::day_17::part_a::part_a(),
                //     Part::B => event_2024::day_17::part_b::part_b(),
                // },
                // 18 => match self.part {
                //     Part::A => event_2024::day_18::part_a::part_a(),
                //     Part::B => event_2024::day_18::part_b::part_b(),
                // },
                // 19 => match self.part {
                //     Part::A => event_2024::day_19::part_a::part_a(),
                //     Part::B => event_2024::day_19::part_b::part_b(),
                // },
                // 20 => match self.part {
                //     Part::A => event_2024::day_20::part_a::part_a(),
                //     Part::B => event_2024::day_20::part_b::part_b(),
                // },
                // 21 => match self.part {
                //     Part::A => event_2024::day_21::part_a::part_a(),
                //     Part::B => event_2024::day_21::part_b::part_b(),
                // },
                // 22 => match self.part {
                //     Part::A => event_2024::day_22::part_a::part_a(),
                //     Part::B => event_2024::day_22::part_b::part_b(),
                // },
                // 23 => match self.part {
                //     Part::A => event_2024::day_23::part_a::part_a(),
                //     Part::B => event_2024::day_23::part_b::part_b(),
                // },
                // 24 => match self.part {
                //     Part::A => event_2024::day_24::part_a::part_a(),
                //     Part::B => event_2024::day_24::part_b::part_b(),
                // },
                // 25 => match self.part {
                //     Part::A => event_2024::day_25::part_a::part_a(),
                //     Part::B => event_2024::day_25::part_b::part_b(),
                // },
                _ => panic!("Invalid day!"),
            },
            _ => panic!("Invalid year!"),
        }
    }
}

fn main() {
    Cli::parse().run();
}
