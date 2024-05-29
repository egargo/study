// https://www.codewars.com/kata/57a06b07cf1fa58b2b000252/train/rust

fn is_it_letter(c: char) -> bool {
    match c.is_alphabetic() {
        true => true,
        _ => false,
    }
}

fn main() {
    println!("{}", is_it_letter('A'));
}
