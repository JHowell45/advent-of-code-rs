use aoc_core::file_reader::get_file_contents;
use event_2025::shared::day5::FreshIngredientDB;

fn main() {
    let fresh_db = FreshIngredientDB::from_str(get_file_contents(2025, 5).as_str());
    println!(
        "The total number of fresh IDs is: {}",
        fresh_db.total_fresh_ids()
    );
}
