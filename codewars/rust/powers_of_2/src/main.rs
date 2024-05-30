// https://www.codewars.com/kata/57a083a57cb1f31db7000028/train/rust

fn powers_of_two(n: u8) -> Vec<u128> {
    if n == 0 {
        return vec![1];
    }

    (0..=n)
        .into_iter()
        .map(|i| u128::pow(2, i.into()))
        .collect::<Vec<u128>>()
}

fn main() {
    assert_eq!(powers_of_two(0), &[1]);
    assert_eq!(powers_of_two(1), &[1, 2]);
    assert_eq!(powers_of_two(4), &[1, 2, 4, 8, 16]);
}
