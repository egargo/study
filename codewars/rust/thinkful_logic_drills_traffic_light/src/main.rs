// https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust

fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow".to_owned(),
        "yellow" => "red".to_owned(),
        "red" => "green".to_owned(),
        _ => panic!(),
    }
}

fn main() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}
