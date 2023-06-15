// https://www.codewars.com/kata/5552101f47fc5178b1000050/train/rust

fn dig_pow(n: i64, p: i32) -> i64 {
    let v = (n.to_string().chars())
        .map(|i| i.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let sum: i64 = v
        .iter()
        .enumerate()
        .map(|(i, j)| i64::pow(*j as i64, (i as i64 + p as i64).try_into().unwrap()))
        .sum::<i64>();

    let k: i64 = (1..=1_000_000).filter(|i| (sum / i) == n).sum();

    if k > 0 {
        return sum / n;
    } else {
        return -1;
    }
}

fn main() {
    println!("{:?}", dig_pow(89, 1)); // 1
    println!("{:?}", dig_pow(92, 1)); // -1
    println!("{:?}", dig_pow(46288, 3)); // 51
    println!("{:?}", dig_pow(46288, 5)); // -1
}
