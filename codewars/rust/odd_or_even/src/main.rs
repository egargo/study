// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust


fn odd_or_even(numbers: Vec<i32>) -> String {
    if numbers.is_empty() {
        return "even".to_string();
    }

    if numbers.iter().sum::<i32>() % 2 == 0 {
        return "even".to_string();
    }

    "odd".to_string()
}

fn main() {
    assert_eq!(odd_or_even(vec![]), "even");
    assert_eq!(odd_or_even(vec![0]), "even");
    assert_eq!(odd_or_even(vec![1]), "odd");
    assert_eq!(odd_or_even(vec![0, 1, 5]), "even");
    assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
    assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
    assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");
}
