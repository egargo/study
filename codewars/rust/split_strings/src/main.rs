// https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/rust

fn solution(s: &str) -> Vec<String> {
    (0..s.len())
        .step_by(2)
        .map(|i| {
            if s.len().ne(&(i + 1)) {
                s[i..i + 2].to_owned()
            } else {
                format!("{}_", s.chars().nth(s.len() - 1).unwrap())
            }
        })
        .collect()
}

fn main() {
    assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
    assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(solution(""), [] as [&str; 0]);
}
