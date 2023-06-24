// https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust


fn reverse_words(words: &str) -> String {
    words.split(' ').rev().collect::<Vec<_>>().join(" ")
}

fn main() {
    assert_eq!(reverse_words("hello world!"), "world! hello");
}
