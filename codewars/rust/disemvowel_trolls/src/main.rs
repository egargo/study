// https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust


use regex::Regex;

fn disemvowel(s: &str) -> String {
    let re = Regex::new(r"(?i)a|e|i|o|u").unwrap();

    re.replace_all(s, "").to_string()
}

fn main() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
