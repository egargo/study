// https://www.codewars.com/kata/534d2f5b5371ecf8d2000a08/train/rust


use std::io;

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {

    let mut table: Vec<Vec<usize>> = Vec::new();
    let mut row: Vec<usize> = Vec::new();

    for i in 1..=len {
        for j in 1..=len {
            row.push(i * j);
        }
        table.push(row.clone());
        row.clear()
    }

    table
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let len: usize = input.trim().parse().expect("Please type a number");

    println!("{:?}", multiplication_table(len));
}
