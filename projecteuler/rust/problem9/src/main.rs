/**
 * GitHub: https://github.com/egargo
 * Completed on: Sun, 18 Jun 2023, 11:12
 *
 * Problem: https://projecteuler.net/problem=9
 */


macro_rules! pow {
    ($i: expr) => {
        {
            $i * $i
        }
    };
}

fn solution() -> i32 {
    for a in 1..=1000 {
        for b in 1..=1000 {
            let c = 1000 - a - b;
            if pow!(a) + pow!(b) == pow!(c) {
                return a * b * c;
            }
        }
    }

    0
}

fn main() {
    println!("{:?}", solution());
}
