use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut occur: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|i| *occur.entry(*i).or_insert(0) += 1);
        **occur
            .iter()
            .filter(|(_, v)| **v == 1)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
}
