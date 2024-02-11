struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .into_iter()
            .map(|i| {
                if i % 3 == 0 && i % 5 == 0 {
                    "FizzBuzz".to_owned()
                } else if i % 3 == 0 {
                    "Fizz".to_owned()
                } else if i % 5 == 0 {
                    "Buzz".to_owned()
                } else {
                    i.to_string()
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(3));
    println!("{:?}", Solution::fizz_buzz(5));
    println!("{:?}", Solution::fizz_buzz(12));
    println!("{:?}", Solution::fizz_buzz(15));
}
