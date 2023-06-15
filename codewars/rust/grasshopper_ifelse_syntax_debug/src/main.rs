// https://www.codewars.com/kata/57089707fe2d01529f00024a/train/rust


fn check_alive(health: i8) -> bool {
    if health > 0 {
        return true;
    }

    false
}


fn main() {
    println!("{}", check_alive(12));
}
