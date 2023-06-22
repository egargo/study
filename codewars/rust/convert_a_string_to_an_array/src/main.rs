// https://www.codewars.com/kata/57e76bc428d6fbc2d500036d/train/rust


fn string_to_array(s: &str) -> Vec<String> {
    s.split(" ").map(| i | i.to_string()).collect::<Vec<String>>()
}

fn main() {
    assert_eq!(string_to_array("Robin Singh"), &["Robin", "Singh"]);
    assert_eq!(string_to_array("CodeWars"), &["CodeWars"]);
    assert_eq!(string_to_array("I love arrays they are my favorite"), &["I", "love", "arrays", "they", "are", "my", "favorite"]);
    assert_eq!(string_to_array("1 2 3"), &["1", "2", "3"]);
}
