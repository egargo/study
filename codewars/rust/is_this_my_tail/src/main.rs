// https://www.codewars.com/kata/56f695399400f5d9ef000af5/train/rust


fn correct_tail(body: &str, tail: char) -> bool {
    if &body[body.len() - 1 ..] == &tail.to_string() {
        return true;
    }

    false
}

fn main() {
    println!("{:?}", correct_tail("Bee", 'e'));
    println!("{:?}", correct_tail("Uggggggggggggggs", 'e'));
}
