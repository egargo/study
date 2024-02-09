#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        Vec::new()
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![2, 1]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![1, 0]);
}
