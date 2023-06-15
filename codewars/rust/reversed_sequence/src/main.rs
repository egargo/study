// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust


fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().map(| i | i).collect::<Vec<u32>>()
}


fn main() {
    println!("{:?}", reverse_seq(12));
}
