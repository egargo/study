// https://www.codewars.com/kata/54fb963d3fe32351f2000102/train/rust

fn collatz(n: u64) -> u64 {
    let mut m = n;
    let mut length = 1;

    while m != 1 {
        match m % 2 {
            0 => m = m / 2,
            _ => m = m * 3 + 1,
        }

        length = length + 1;
    }

    length
}

fn main() {
    assert_eq!(collatz(20), 8);
    assert_eq!(collatz(15), 18);
}
