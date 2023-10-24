#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}

fn main() {
    assert_eq!(Solution::to_lower_case("Bee".to_owned()), "bee".to_owned());
}
