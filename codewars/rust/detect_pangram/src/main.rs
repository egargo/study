// https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/rust

use std::collections::HashMap;

fn is_pangram(s: &str) -> bool {
    let mut letters: HashMap<char, u8> = HashMap::new();

    for i in s.to_lowercase().chars() {
        if i.is_alphabetic() && !letters.get(&i).is_some() {
            letters.insert(i, 1);
        }
    }

    match letters.len() {
        26 => true,
        _ => false,
    }
}

fn main() {
    assert_eq!(
        is_pangram("The quick, brown fox jumps over the lazy dog!"),
        true
    );
    assert_eq!(is_pangram("Cwm fjord bank glyphs vext quiz"), true);
    assert_eq!(is_pangram("Pack my box with five dozen liquor jugs."), true);
    assert_eq!(is_pangram("How quickly daft jumping zebras vex."), true);
    assert_eq!(is_pangram("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ"), true);
    assert_eq!(is_pangram("This isn't a pangram!"), false);
    assert_eq!(is_pangram("abcdefghijklmopqrstuvwxyz"), false);
    assert_eq!(is_pangram("Aacdefghijklmnopqrstuvwxyz"), false);
}
