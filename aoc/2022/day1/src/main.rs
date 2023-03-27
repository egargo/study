use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(fpath: &str) -> Vec<Vec<i32>> {
    let file = File::open(fpath)
        .expect("Could not read file");

    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<i32>> = Vec::new();
    let mut temp: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        if line.trim().is_empty() {
            lines.push(temp.clone());
            temp.clear();
        } else {
            temp.push(line.trim().parse::<i32>().unwrap());
        }
    }

    lines
}

fn main() {
    let cals = read_file("input.txt");

    let scores: Vec<i32> = cals.iter()
        .map(|row| row.iter().sum())
        .collect();

    println!("{:?}", scores.iter().max().unwrap());
}
