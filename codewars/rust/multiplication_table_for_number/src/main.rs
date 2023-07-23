// https://www.codewars.com/kata/5a2fd38b55519ed98f0000ce/train/rust

fn multi_table(n: u64) -> String {
    (1..=10)
        .map(|i| format!("{} * {} = {}", i, n, (i * n)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!("{:?}", multi_table(12));
}
