struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // let zeroes = nums.iter().filter(|i| **i == 0).collect::<Vec<_>>().len();
        let zeroes = nums.iter().filter(|i| **i == 0).count();
        nums.retain_mut(|i| if *i != 0 { true } else { false });
        nums.append(&mut vec![0; zeroes]);
        println!("{:?}", nums);
    }
}

fn main() {
    Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12]);
    Solution::move_zeroes(&mut vec![0]);
}
