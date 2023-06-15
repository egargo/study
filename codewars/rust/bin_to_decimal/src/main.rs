// https://www.codewars.com/kata/57a5c31ce298a7e6b7000334/train/rust


fn bin_to_decimal(inp: &str) -> i32 {
  i32::from_str_radix(inp, 2).unwrap()
}

fn main() {
  println!("{:?}", bin_to_decimal(&"1001001"));
}
