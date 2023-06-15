// https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/rust


fn summation(n: i32) -> i32 {
    (1..=n).map(| i | i).sum()
}

fn main() {
    println!("{}", summation(12));
}
