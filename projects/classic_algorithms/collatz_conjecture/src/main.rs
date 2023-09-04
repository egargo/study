use std::io;

fn collatz(mut n: u32) -> (Vec<u32>, u32) {
    let mut count: u32 = 1;
    let mut sequence = Vec::new();

    while n > 1 {
        match n % 2 {
            0 => {
                n = n / 2;
                count += 1;
            }
            _ => {
                n = 3 * n + 1;
                count += 1;
            }
        }
        sequence.push(n);
    }

    (sequence, count)
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Not a number");

    println!("{:?}", collatz(n));
}
