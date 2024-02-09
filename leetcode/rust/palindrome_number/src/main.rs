struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let y = x.to_string().chars().rev().collect::<Vec<char>>();
        let x = x.to_string().chars().collect::<Vec<char>>();

        matches!(x.eq(&y), true)
    }
}

fn main() {
    for i in vec![121, -121, 10] {
        let s = Solution::is_palindrome(i);
        println!("{}", s);
    }
}
