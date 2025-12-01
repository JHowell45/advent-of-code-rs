use std::fs;
use std::path::PathBuf;

pub fn get_file_contents(year: i32, day: i32) -> String {
    let local_path: String = format!("./event_{year}/puzzle_inputs/day{day}.txt");
    // println!("{}", local_path);
    // println!("{:?}", fs::canonicalize(".").unwrap());

    let file_path = fs::canonicalize(PathBuf::from(&local_path)).unwrap();

    return fs::read_to_string(&file_path).expect("Should have been able to read the file");
}
