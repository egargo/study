// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust

use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut counter: HashMap<char, u32> = HashMap::new();

    for i in text.to_lowercase().chars() {
        if !i.is_whitespace() {
            *counter.entry(i).or_insert(0) += 1;
        }
    }

    counter
        .values()
        .filter(|i| *i > &1)
        .map(|_| 1)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn main() {
    assert_eq!(count_duplicates("abcde"), 0);
    assert_eq!(count_duplicates("abcdea"), 1);
    assert_eq!(count_duplicates("indivisibility"), 1);
}
