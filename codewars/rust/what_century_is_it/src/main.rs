// https://www.codewars.com/kata/52fb87703c1351ebd200081f/train/rust

fn ordinal(century: u32) -> &'static str {
    match century {
        11 | 12 | 13 => "th",
        _ => match century % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

fn what_century(year: &str) -> String {
    let yy: u32 = year[0..2].parse().unwrap();

    match year[2..4].parse().unwrap() {
        0 => format!("{}{}", yy, ordinal(yy)),
        _ => format!("{}{}", yy + 1, ordinal(yy + 1)),
    }
}

fn main() {
    assert_eq!(what_century("1999"), "20th");
    assert_eq!(what_century("2011"), "21st");
    assert_eq!(what_century("2154"), "22nd");
    assert_eq!(what_century("2259"), "23rd");
    assert_eq!(what_century("1234"), "13th");
    assert_eq!(what_century("1023"), "11th");
    assert_eq!(what_century("2000"), "20th");
    assert_eq!(what_century("3210"), "33rd");
}
