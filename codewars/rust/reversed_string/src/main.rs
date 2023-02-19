// https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/rust


fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect::<String>().to_string()
}

fn main() {
    println!("{}", solution("codewars"));
}
