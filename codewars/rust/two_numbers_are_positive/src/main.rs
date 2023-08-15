// https://www.codewars.com/kata/602db3215c22df000e8544f0/train/rust

fn check_number(n: i32) -> i32 {
    if n > 0 {
        1
    } else {
        0
    }
}

fn two_are_positive(a: i32, b: i32, c: i32) -> bool {
    let mut check = check_number(a);
    check += check_number(b);
    check += check_number(c);

    if check == 2 {
        true
    } else {
        false
    }
}

fn main() {
    assert_eq!(two_are_positive(2, 4, -3), true);
    assert_eq!(two_are_positive(-4, 6, 8), true);
    assert_eq!(two_are_positive(4, -6, 9), true);
    assert_eq!(two_are_positive(-4, 6, 0), false);
    assert_eq!(two_are_positive(4, 6, 10), false);
    assert_eq!(two_are_positive(-14, -3, -4), false);
    assert_eq!(two_are_positive(44, 1, -50), true);
}
