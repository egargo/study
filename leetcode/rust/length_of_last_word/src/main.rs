struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .len() as i32
    }
}

fn main() {
    let words = vec![
        "Hello World",
        "   fly me   to   the moon  ",
        "luffy is still joyboy",
    ];

    for i in words {
        let s = Solution::length_of_last_word(i.to_owned());
        println!("{}", s);
    }
}
