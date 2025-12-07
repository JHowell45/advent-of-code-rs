use aoc_core::file_reader::get_file_contents;

struct FreshRange(u64, u64);

fn total_fresh_ids(ranges: &str) -> u64 {
    let parsed_ranges: Vec<FreshRange> = Vec::new();
    for range in ranges.lines() {
        let parsed_range: Vec<u64> = range.split("-").map(|v| v.parse::<u64>().unwrap()).collect();
    }
    parsed_ranges.iter().map(|range| range.1 - range.0).sum()
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
        assert_eq!(total_fresh_ids(data), 14);
    }
}