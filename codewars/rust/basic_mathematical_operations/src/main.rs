// https://www.codewars.com/kata/57356c55867b9b7a60000bd7/train/rust

fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        '/' => value1 / value2,
        _ => panic!(),
    }
}

fn main() {
    assert_eq!(basic_op('+', 4, 7), 11);
    assert_eq!(basic_op('-', 15, 18), -3);
    assert_eq!(basic_op('*', 5, 5), 25);
    assert_eq!(basic_op('/', 49, 7), 7);
}
