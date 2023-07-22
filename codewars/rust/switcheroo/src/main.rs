// https://www.codewars.com/kata/57f759bb664021a30300007d/train/rust


fn switcheroo(s: &str) -> String {
    let mut switch = String::new();

    for i in s.chars() {
        match i {
            'a' => switch.push_str("b"),
            'b' => switch.push_str("a"),
            _ => switch.push_str(&i.to_string()),
        }
    }

    switch
}

fn main() {
    assert_eq!(switcheroo("abc"), "bac");
    assert_eq!(switcheroo("aaabcccbaaa"), "bbbacccabbb");
    assert_eq!(switcheroo("ccccc"), "ccccc");
    assert_eq!(switcheroo("abababababababab"), "babababababababa");
    assert_eq!(switcheroo("aaaaa"), "bbbbb");
}
