use aoc_core::file_reader::get_file_contents;

struct IdRange(u64, u64);

struct FreshIngredientDB {
    id_ranges: Vec<IdRange>
}

impl FreshIngredientDB {
    fn from_str(ingredients_list: &str) -> Self {
        Self {
            id_ranges: ingredients_list.lines().map(|line| {
                let x: Vec<u64> = line.split("-").map(|v| v.parse::<u64>().unwrap()).collect();
                IdRange(x[0], x[1])
            }).collect::<Vec<IdRange>>()
        }
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        for range in self.id_ranges.iter() {
            if range.0 <= id &&  range.1 >= id {
                return true;
            }
        }
        false
    }
}

fn fresh_ingredient_sum(input: String) -> u32 {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let fresh_db = FreshIngredientDB::from_str(data[0]);
    data[1].lines().map(|l| {
        let v: u64 = l.trim().parse().unwrap();
        fresh_db.is_fresh(v) as u32
    }).sum()
}

fn main() {
    println!("The total number of fresh ingredients is: {}", fresh_ingredient_sum(get_file_contents(2025, 5)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, false)]
    #[case(5, true)]
    #[case(8, false)]
    #[case(11, true)]
    #[case(17, true)]
    #[case(32, false)]
    fn example(#[case] check_id: u64, #[case] expected: bool) {
        let fresh_db: FreshIngredientDB = FreshIngredientDB::from_str(
            "3-5
10-14
16-20
12-18"
        );
        assert_eq!(fresh_db.is_fresh(check_id), expected);
    }

    #[rstest]
    #[case("3-5
10-14
16-20
12-18

1
5
8
11
17
32", 3)]
    fn test_fresh_ingredient_sum(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(fresh_ingredient_sum(String::from(input)), expected);
    }
}