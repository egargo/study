// https://www.codewars.com/kata/57a2013acf1fa5bfc4000921/train/rust


fn find_average(slice: &[f64]) -> f64 {
	let length = slice.len() as u64;

	if length == 0 {
		return 0.0
	}

	let mut sum: f64 = 0.0;

	for i in slice {
		sum += i;
		println!("{}", sum);
	}

	sum / length as f64
}

fn main() {

	let slice = vec![17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,];

	println!("{}", find_average(&slice));
}
