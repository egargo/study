// https://www.codewars.com/kata/53369039d7ab3ac506000467/train/rust


fn bool_to_word(value: bool) -> &'static str {
	return if value == true { "Yes" } else { "No" };
}


fn main() {
	println!("{}", bool_to_word(true));
	println!("{}", bool_to_word(false));
}
