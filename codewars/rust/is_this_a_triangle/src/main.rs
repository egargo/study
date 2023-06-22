// https://www.codewars.com/kata/56606694ec01347ce800001b/train/rust


fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if (a + b) > c && (a + c) > b && (b + c) > a {
        return true;
    }

    false
}

fn main() {
    assert_eq!(is_triangle(1, 2, 2), true);
    assert_eq!(is_triangle(7, 2, 2), false);
    assert_eq!(is_triangle(1, 2, 3), false);
    assert_eq!(is_triangle(1, 3, 2), false);
    assert_eq!(is_triangle(3, 1, 2), false);
    assert_eq!(is_triangle(5, 1, 2), false);
    assert_eq!(is_triangle(1, 2, 5), false);
    assert_eq!(is_triangle(2, 5, 1), false);
    assert_eq!(is_triangle(4, 2, 3), true);
    assert_eq!(is_triangle(5, 1, 5), true);
    assert_eq!(is_triangle(2, 2, 2), true);
    assert_eq!(is_triangle(-1, 2, 3), false);
    assert_eq!(is_triangle(1, -2, 3), false);
    assert_eq!(is_triangle(1, 2, -3), false);
    assert_eq!(is_triangle(0, 2, 3), false);
}
