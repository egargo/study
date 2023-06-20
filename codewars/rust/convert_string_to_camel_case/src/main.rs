// https://www.codewars.com/kata/517abf86da9663f1d2000003/train/rust


use regex::Regex;

macro_rules! title {
    ($text: expr) => {{
        $text.chars().nth(0).unwrap().to_string().to_uppercase()
            + &$text[1..$text.len()].to_string()
    }};
}

fn to_camel_case(text: &str) -> String {
    let re = Regex::new(r"[-|_]+").unwrap();

    re.split(text)
        .enumerate()
        .map(|(i, j)| {
            if i == 0 {
                j.to_owned()
            } else {
                title!(j.to_owned())
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    assert_eq!(to_camel_case("the_stealth_warrior"), "theStealthWarrior");
    assert_eq!(to_camel_case("The-Stealth-Warrior"), "TheStealthWarrior");
    assert_eq!(to_camel_case("A-B-C"), "ABC");
}
