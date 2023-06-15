// https://www.codewars.com/kata/596fba44963025c878000039/train/rust


fn contamination(text: &str, character: &str) -> String {
    (0..text.len()).map(|_| character).collect::<String>()
}

fn main() {
    println!("{:?}", contamination("abc", "z"));
}
