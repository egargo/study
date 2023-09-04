use std::io;

fn binary_search_iterative(numbers: Vec<i32>, target: i32) -> Result<i32, i32> {
    let mut left = 0;
    let mut right = numbers.len();

    while left <= right {
        let midpoint = left + (right - left) / 2;

        if numbers[midpoint] == target {
            return Ok(target);
        } else {
            if target < numbers[midpoint] {
                right = midpoint - 1;
            } else {
                left = midpoint + 1;
            }
        }
    }

    Err(-1)
}

fn main() {
    let numbers: Vec<i32> = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

    let mut target = String::new();

    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    let target: i32 = target.trim().parse::<i32>().unwrap();

    let result = binary_search_iterative(numbers, target);

    match result {
        Ok(_) => println!("Result: {:?}", result.unwrap()),
        Err(_) => println!("No result found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = binary_search_iterative(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 12);
        assert_eq!(numbers, Ok(12));

        let numbers =
            binary_search_iterative(vec![-12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1], -12);
        assert_eq!(numbers, Ok(-12));
    }
}
