// https://www.codewars.com/kata/54bf85e3d5b56c7a05000cf9/train/rust


fn number(lines: &[&str]) -> Vec<String> {
  lines
    .iter()
    .enumerate()
    .map(| (i, j) | (i+1).to_string() + ": " + j )
    .collect::<Vec<_>>()
}

fn main() {
  let lines = vec!["a", "b", "c"];
  println!("{:?}", number(&lines));
}
