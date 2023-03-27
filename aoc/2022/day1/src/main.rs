mod read;

fn aoc1p1(cals: Vec<Vec<i32>>) -> i32 {
    *cals.iter()
        .map(|row| row.iter().sum())
        .collect::<Vec<_>>().iter().max().unwrap()
}

fn aoc1p2(cals: Vec<Vec<i32>>) -> i32 {
    let mut total: Vec<i32> = cals.iter()
        .map(|row| row.iter().sum())
        .collect();

    total.sort_by(|a, b| b.cmp(a));

    total[0..3].iter().sum()
}

fn main() {
    let cals = read::read_file("input.txt");

    println!("{:?}", aoc1p1(cals.clone()));
    println!("{:?}", aoc1p2(cals.clone()));
}
