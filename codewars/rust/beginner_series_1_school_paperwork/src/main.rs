// https://www.codewars.com/kata/55f9b48403f6b87a7c0000bd/train/rust


fn paperwork(n: i16, m:i16) -> u32 {
    if n > 0 && m > 0 {
        return (n * m) as u32;
    }

    0
}

fn main() {
    assert_eq!(paperwork(5,5), 25);
    assert_eq!(paperwork(5,-5), 0);
    assert_eq!(paperwork(-5,-5), 0);
    assert_eq!(paperwork(-5,5), 0);
    assert_eq!(paperwork(5,0), 0);
}

