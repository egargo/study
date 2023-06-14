use std::io;

fn solution(n1: u32, n2: u32) -> u32 {
  n1 + n2
}

fn input() -> u32 {
  let mut n = String::new();

  io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");

  n.trim().parse().expect("Not a number")
}

fn main() {
  let n1 = input();
  let n2 = input();

  println!("{:?}", solution(n1, n2));
}
