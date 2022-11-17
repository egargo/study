// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust


use read_input::prelude::*;


fn fill(hms: u32) -> String {
	if hms.to_string().len() == 1 {
		return format!("{:02}", hms);
	}

	hms.to_string()
}


fn make_readable(seconds: u32) -> String {
	let hour = seconds / 3600;
	let mins = (seconds % 3600) / 60;
	let secs = (seconds % 60) % 60;

	format!("{}:{}:{}", fill(hour), fill(mins), fill(secs))
}


fn main() {
	let seconds = input::<u32>().get();
    println!("{}", make_readable(seconds));
}
