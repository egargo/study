// https://www.codewars.com/kata/582cb0224e56e068d800003c/train/rust


fn litres(time: f64) -> i32 {
    (time * 0.5) as i32
}

fn main() {
    println!("{:?}", litres(12.0));
}
