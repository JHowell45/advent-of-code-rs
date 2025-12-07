#[derive(Clone, Copy)]
pub struct FreshRange(u64, u64);

pub struct FreshIngredientDB {
    id_ranges: Vec<FreshRange>,
}

impl FreshIngredientDB {
    pub fn from_str(ingredients_list: &str) -> Self {
        Self {
            id_ranges: ingredients_list
                .lines()
                .map(|line| {
                    let x: Vec<u64> = line.split("-").map(|v| v.parse::<u64>().unwrap()).collect();
                    FreshRange(x[0], x[1])
                })
                .collect::<Vec<FreshRange>>(),
        }
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        for range in self.id_ranges.iter() {
            if range.0 <= id && range.1 >= id {
                return true;
            }
        }
        false
    }

    pub fn total_fresh_ids(&self) -> u64 {
        let mut fresh_ranges: Vec<FreshRange> = Vec::new();
        for range in self.id_ranges.iter() {
            
        }
        fresh_ranges.iter().map(|range| range.1 - range.0).sum()
    }
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
12-18",
        );
        assert_eq!(fresh_db.is_fresh(check_id), expected);
    }

    #[test]
    fn test_total_fresh_ids() {
        let fresh_db: FreshIngredientDB = FreshIngredientDB::from_str(
            "3-5
10-14
16-20
12-18",
        );
        assert_eq!(fresh_db.total_fresh_ids(), 14);
    }
}
