// https://www.codewars.com/kata/5265326f5fda8eb1160004c8/train/rust


fn number_to_string(i: i32) -> String {
    i.to_string()
}

fn main() {
    assert_eq!(number_to_string(67), "67");
    assert_eq!(number_to_string(79585), "79585");
    assert_eq!(number_to_string(1+2), "3");
    assert_eq!(number_to_string(1-2), "-1");
}
