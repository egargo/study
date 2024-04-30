// https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/rust

use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    let mut count: HashMap<char, u8> = HashMap::new();

    for i in word.to_lowercase().chars() {
        *count.entry(i).or_insert(0) += 1;
    }

    let mut encoded = String::new();

    for i in word.chars() {
        let check = count.get(&i);

        if check.is_some() && *check.unwrap() == 1 {
            encoded.push_str("(");
        } else {
            encoded.push_str(")");
        }
    }

    encoded
}

fn main() {
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
