// https://www.codewars.com/kata/5a9c35e9ba1bb5c54a0001ac/train/rust

fn add(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;

    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }

    a
}

fn main() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(5, 19), 24);
    assert_eq!(add(23, 17), 40);
    assert_eq!(add(-14, -16), -30);
    assert_eq!(add(-50, -176), -226);
    assert_eq!(add(-10, -29), -39);
    assert_eq!(add(-13, 13), 0);
    assert_eq!(add(-27, 18), -9);
    assert_eq!(add(-90, 30), -60);
}
