struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        *accounts
            .iter()
            .map(|i| i.iter().sum::<i32>())
            .collect::<Vec<_>>()
            .iter()
            .max()
            .unwrap()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]])
    );
}
