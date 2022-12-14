// https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust

use std::io;

fn dna_strand(dna: &str) -> String {
    let mut strand = String::new();

    for c in dna.chars() {
        match c {
            'A' => strand.push_str("T"),
            'T' => strand.push_str("A"),
            'G' => strand.push_str("C"),
            'C' => strand.push_str("G"),
            _ => (),
        }
    }

    strand
}

fn main() {
    let mut dna = String::new();

    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    println!("{}", dna_strand(&dna));
}
