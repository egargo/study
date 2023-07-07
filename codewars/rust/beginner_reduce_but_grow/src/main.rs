// https://www.codewars.com/kata/57f780909f7e8e3183000078/train/rust


fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

fn main() {
    assert_eq!(grow(vec![1, 2, 3]), 6);
    assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
    assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
}
