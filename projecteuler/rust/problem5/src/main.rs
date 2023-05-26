fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn solution(n: u128) -> u128 {
    let mut b: u128 = 1;

    (1..=n).map(|a| b *= a / gcd(a, b)).for_each(move |i| i);

    b
}

fn main() {
    println!("{:?}", solution(20));
}
