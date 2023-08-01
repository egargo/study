// https://www.codewars.com/kata/555086d53eac039a2a000083/train/rust

fn lovefunc(flower1: u16, flower2: u16) -> bool {
    if flower1 % 2 == 0 && flower2 % 2 != 0 || flower1 % 2 != 0 && flower2 % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    assert_eq!(lovefunc(1, 4), true);
    assert_eq!(lovefunc(2, 2), false);
    assert_eq!(lovefunc(0, 1), true);
    assert_eq!(lovefunc(0, 0), false);
}
