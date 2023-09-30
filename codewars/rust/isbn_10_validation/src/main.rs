// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001/train/rust

fn valid_isbn10(isbn: &str) -> bool {
    const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWYZ";

    let first = isbn.chars().nth(0).unwrap();
    let last = isbn.chars().nth(isbn.len() - 1).unwrap();

    if isbn.len() != 10 {
        return false;
    } else {
        if LETTERS.contains(first) || LETTERS.contains(last) || first == 'X' {
            return false;
        }
    }

    let mut isbn10: Vec<u32> = vec![];

    for i in isbn.chars() {
        if i == 'X' {
            isbn10.push(10);
        } else {
            isbn10.push(i.to_digit(10).unwrap());
        }
    }

    let mut count = 1;
    let mut sum = 0;

    for i in isbn10.iter() {
        sum = sum + i * count;
        count += 1;
    }

    match sum % 11 == 0 {
        true => true,
        false => false,
    }
}

fn main() {
    assert_eq!(valid_isbn10("1112223339"), true);
    assert_eq!(valid_isbn10("048665088X"), true);
    assert_eq!(valid_isbn10("1293000000"), true);
    assert_eq!(valid_isbn10("1234554321"), true);
    assert_eq!(valid_isbn10("1234512345"), false);
    assert_eq!(valid_isbn10("1293"), false);
    assert_eq!(valid_isbn10("X123456788"), false);
    assert_eq!(valid_isbn10("ABCDEFGHIJ"), false);
    assert_eq!(valid_isbn10("XXXXXXXXXX"), false);
    assert_eq!(valid_isbn10("123456789T"), false);
}
