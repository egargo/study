// https://www.codewars.com/kata/57a0885cbb9944e24c00008e/train/rust

use regex::Regex;

fn remove_exclamation_marks(input: &str) -> String {
    Regex::new("[!]")
        .unwrap()
        .replace_all(input, "")
        .to_string()
}

fn main() {
    assert_eq!(remove_exclamation_marks("Hello World!"), "Hello World");
    assert_eq!(remove_exclamation_marks("Hello World!!!"), "Hello World");
    assert_eq!(remove_exclamation_marks("Hi! Hello!"), "Hi Hello");
    assert_eq!(remove_exclamation_marks(""), "");
    assert_eq!(remove_exclamation_marks("Oh, no!!!"), "Oh, no");
}
