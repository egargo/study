// https://www.codewars.com/kata/59a8570b570190d313000037/train/rust

fn sum_cubes(n: u32) -> u32 {
    let digits: Vec<u32> = Vec::from_iter(1..=n)
        .iter()
        .map(|i| u32::pow(*i, 3))
        .collect();

    digits.iter().sum()
}

fn main() {
    println!("{:?}", sum_cubes(12));
}
