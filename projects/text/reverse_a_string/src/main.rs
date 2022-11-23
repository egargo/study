use std::io;

fn reverse_string(input: &str) -> String {
	input.to_string().chars().rev().collect::<String>()
}

fn main() {
	let mut input = String::new();

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");

	println!("{}", reverse_string(&input));
}
