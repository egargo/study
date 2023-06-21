// https://www.codewars.com/kata/55ad04714f0b468e8200001c/train/rust


fn get_char(c: i32) -> char {
    char::from_u32(c as u32).unwrap()
}

fn main() {
    assert_eq!(get_char(66), 'B');
    assert_eq!(get_char(101), 'e');
    assert_eq!(get_char(101), 'e');
}
