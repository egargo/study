// https://www.codewars.com/kata/5286b2e162056fd0cb000c20/train/rust


fn collatz(mut n: u32) -> String {
  let mut s = n.to_string() + "->";

  while n != 1 as u32 {
    if n % 2 == 0 {
      n /= 2;
    } else {
      n = n * 3 + 1;
    }

    s += &(n.to_string() + "->");
  }

  s[0..s.len()-2].to_string()
}

fn main() {
    println!("{:?}", collatz(3));
}
