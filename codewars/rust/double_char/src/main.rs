// https://www.codewars.com/kata/56b1f01c247c01db92000076/train/rust

fn double_char(s: &str) -> String {
    s.chars()
        .map(|c| c.to_string().repeat(2))
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    assert_eq!(double_char("Hello World"), "HHeelllloo  WWoorrlldd");
    assert_eq!(double_char("1234!_ "), "11223344!!__  ");
}
