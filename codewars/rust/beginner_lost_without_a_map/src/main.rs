fn maps(values: &Vec<i32>) -> Vec<i32> {
	let mut doubled = Vec::new();

	for i in values {
		doubled.push(i * 2);
	}

	doubled
}


fn main() {
	let values = vec![1, 2, 3];

	println!("{:?}", maps(&values));
}
