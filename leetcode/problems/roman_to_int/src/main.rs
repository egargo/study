use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let numbers: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut int = 0;
        let roman = s.chars().collect::<Vec<char>>();

        for i in 0..roman.len() {
            let current = numbers.get(&roman[i]).unwrap_or(&0);
            match roman.len().ne(&(i + 1)) {
                true => {
                    let next = numbers.get(&roman[i + 1]).unwrap_or(&0);

                    match current.ge(next) {
                        true => int += current,
                        false => int -= current,
                    }
                }
                false => int += current,
            }
        }

        int
    }
}

fn main() {
    let roman = vec!["III".to_owned(), "LVIII".to_owned(), "MCMXCIV".to_owned()];

    for i in roman {
        let s = Solution::roman_to_int(i);
        println!("{}", s);
    }
}
