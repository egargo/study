use std::io;

fn search(numbers: Vec<i32>, target: i32) -> Result<i32, i32> {
    for i in numbers.iter() {
        if *i == target {
            return Ok(*i);
        }
    }

    Err(-1)
}

fn main() {
    let numbers: Vec<i32> = Vec::from([-22, -12, -2, 0, 2, 12, 22]);

    let mut target = String::new();

    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read line");

    let target: i32 = target.trim().parse::<i32>().unwrap();

    let result = search(numbers, target);

    match result {
        Ok(_) => println!("Result: {:?}", result.unwrap()),
        Err(_) => println!("No result found"),
    }
}
