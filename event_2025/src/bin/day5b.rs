use aoc_core::file_reader::get_file_contents;


fn total_fresh_ids(ranges: &str) -> u32 {

}

fn main() {
    println!("The total number of fresh IDs is: {}", total_fresh_ids(get_file_contents(2025, 5).as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example() {
        let data: &str = "3-5
10-14
16-20
12-18";
        assert_eq!(total_fresh_ids(), 14);
    }
}