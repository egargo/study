// https://www.codewars.com/kata/52eb114b2d55f0e69800078d/train/rust

use std::collections::HashMap;

struct Cipher {
    map1: String,
    map2: String,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            map1: map1.to_string(),
            map2: map2.to_string(),
        }
    }

    fn helper(&self, map: HashMap<char, char>, string: &str) -> String {
        let mut encoded = String::new();

        for c in string.chars() {
            if map.contains_key(&c) {
                encoded.push_str(&map.get(&c).unwrap().to_string());
            } else {
                encoded.push_str(&c.to_string());
            }
        }

        encoded
    }

    fn encode(&self, string: &str) -> String {
        let mapped: HashMap<char, char> = self
            .map1
            .chars()
            .enumerate()
            .map(|(i, c)| (c, self.map2.chars().nth(i).unwrap()))
            .collect::<HashMap<char, char>>();

        self.helper(mapped, string)
    }

    fn decode(&self, string: &str) -> String {
        let unmapped: HashMap<char, char> = self
            .map2
            .chars()
            .enumerate()
            .map(|(i, c)| (c, self.map1.chars().nth(i).unwrap()))
            .collect::<HashMap<char, char>>();

        self.helper(unmapped, string)
    }
}

fn main() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    println!("{}", cipher.encode("bee"));
    println!("{}", cipher.decode("tii"));
}
