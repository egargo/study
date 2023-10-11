// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust

pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_owned()
}

fn main() {
    remove_char("eloquent");
}
