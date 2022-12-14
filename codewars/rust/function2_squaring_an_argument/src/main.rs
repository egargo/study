// https://www.codewars.com/kata/523b623152af8a30c6000027/train/rust


use std::io;


fn square(n: i32) -> i32 {
    n * n
}


fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim()
        .parse()
        .expect("Please type a number");

    println!("{}", square(n));
}
