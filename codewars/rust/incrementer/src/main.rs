// https://www.codewars.com/kata/590e03aef55cab099a0002e8/train/rust


fn incrementer(nums: &[u32]) -> Vec<u32> {
	println!("{:?}", nums);

	let mut incremented: Vec<u32> = Vec::new();

	for(i, j) in nums.iter().enumerate() {
		incremented.push(((i as u32 + 1) + j) % 10);
	}

	incremented
}


fn main() {
    println!("{:?}", incrementer(&[2, 0, 2, 2, 0, 2, 1, 2]));
}
