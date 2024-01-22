// https://www.codewars.com/kata/55685cd7ad70877c23000102/train/rust

use std::ops::Neg;

fn make_negative(number: i32) -> i32 {
    match number.lt(&0) {
        true => number,
        false => number.neg(),
    }
}

fn main() {
    assert_eq!(make_negative(390), -390);
    assert_eq!(make_negative(1), -1);
    assert_eq!(make_negative(-5), -5);
    assert_eq!(make_negative(0), 0);
}
