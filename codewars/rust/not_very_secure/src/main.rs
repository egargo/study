// https://www.codewars.com/kata/526dbd6c8c0eb53254000110/train/rust


use regex::Regex;

fn alphanumeric(password: &str) -> bool {
    let re = Regex::new(r"(^[[:alnum:]]+$)").unwrap();

    match re.find(password) {
        Some(_) => return true,
        _ => return false,
    }
}

fn main() {
    assert_eq!(alphanumeric("hello world_"), false);
    assert_eq!(alphanumeric("PassW0rd"), true);
    assert_eq!(alphanumeric("    "), false);
}
