// https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/rust


use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for i in input.chars() {
        *counts.entry(i).or_insert(0) += 1;
    }

    counts
}

fn main() {
    println!("{:?}", count("bee"));
}
