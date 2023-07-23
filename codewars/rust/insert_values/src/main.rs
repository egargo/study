// https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust

fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|i| i * (-1)).collect::<Vec<i32>>()
}

fn main() {
    assert_eq!(invert(&vec![1, 2, 3, 4, 5]), vec![-1, -2, -3, -4, -5]);
    assert_eq!(invert(&vec![1, -2, 3, -4, 5]), vec![-1, 2, -3, 4, -5]);
    assert_eq!(invert(&vec![]), vec![]);
}
