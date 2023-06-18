/**
 * GitHub: https://github.com/egargo
 * Completed on: Wed, 24 May 2023, 16:40
 *
 * Problem: https://projecteuler.net/problem=7
 * Formula: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
 */

fn prime(n: u128) -> u128 {
    let mut a: Vec<bool> = vec![true; (n + 1) as usize];

    for i in 2..=n {
        if a[i as usize] == true {
            for j in (i * i..=n).step_by(i as usize) {
                a[j as usize] = false;
            }
        }
    }

    a.iter()
        .enumerate()
        .filter(|(i, j)| *i >= 2 && *j == &true)
        .map(|(i, _)| i as u128)
        .collect::<Vec<u128>>()[10_001 - 1]
}

fn main() {
    let range = 1_000_000;
    println!("{:?}", prime(range))
}
