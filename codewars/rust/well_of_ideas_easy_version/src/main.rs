// https://www.codewars.com/kata/57f222ce69e09c3630000212/train/rust

fn well(x: &[&str]) -> &'static str {
    match x
        .iter()
        .filter(|s| s.eq(&&"good"))
        .collect::<Vec<&&str>>()
        .len()
    {
        0 => "Fail!",
        1 | 2 => "Publish!",
        _ => "I smell a series!",
    }
}

fn main() {
    assert_eq!(well(&["bad", "bad", "bad"]), "Fail!");
    assert_eq!(well(&["good", "bad", "bad", "bad"]), "Publish!");
    assert_eq!(
        well(&["good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good"]),
        "I smell a series!"
    );
}
