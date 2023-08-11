// https://www.codewars.com/kata/5a5f3034cadebf76db000023/train/rust

fn show_bits(n: i32) -> [u8; 32] {
    (0..32)
        .rev()
        .map(|i| if (n & (1 << i)) != 0 { 1 } else { 0 })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn main() {
    println!("{:?}", show_bits(12));
}
