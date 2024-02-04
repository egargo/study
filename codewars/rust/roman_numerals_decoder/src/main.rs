// https://www.codewars.com/kata/51b6249c4612257ac0000005/train/rust

use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {
    println!("{}", roman);

    let map: HashMap<char, i64> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let rroman = roman.chars().map(|i| i).collect::<Vec<char>>();

    let mut num: i64 = 0;

    for i in 0..rroman.len() {
        let current = map.get(&rroman[i]).unwrap_or(&0);
        match roman.len().ne(&(i + 1)) {
            true => {
                let next = map.get(&rroman[i + 1]).unwrap_or(&0);
                match current.ge(next) {
                    true => num += current,
                    _ => num -= current,
                }
            }
            false => num += current,
        }
    }

    num as u64
}

fn main() {
    assert_eq!(roman_as_num(""), 0);
    assert_eq!(roman_as_num("I"), 1);
    assert_eq!(roman_as_num("XXI"), 21);
    assert_eq!(roman_as_num("MMVIII"), 2008);
    assert_eq!(roman_as_num("MDCLXVI"), 1666);
    assert_eq!(roman_as_num("IV"), 4);
    assert_eq!(roman_as_num("CDXLIV"), 444);
}
