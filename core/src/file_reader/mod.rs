use crate::enums::Part;
use std::fs;
use std::path::PathBuf;

pub fn get_file_contents(year: i32, day: i32, part: Part) -> String {
    let part = match part {
        Part::A => 'a',
        Part::B => 'b',
    };
    let local_path: String = format!("./event_{year}/src/day_{day}/puzzle_{part}.txt");    
    let file_path = fs::canonicalize(PathBuf::from(&local_path)).unwrap();
    return fs::read_to_string(&file_path).expect("Should have been able to read the file");
}
