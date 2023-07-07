// https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust


fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|i| i >= &&0).map(|i| i).sum()
}

fn main() {
    assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
    assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
}
