// https://www.codewars.com/kata/55fab1ffda3e2e44f00000c6/train/rust

fn cockroach_speed(s: f64) -> i64 {
    (s / 0.036) as i64
}

fn main() {
    assert_eq!(cockroach_speed(1.08), 30);
    assert_eq!(cockroach_speed(1.09), 30);
    assert_eq!(cockroach_speed(0.0), 0);
}
