// https://www.codewars.com/kata/5355a811a93a501adf000ab7/train/rust

fn fizz_buzz_custom_solver(
    string_one: &str,
    string_two: &str,
    num_one: usize,
    num_two: usize,
) -> Vec<String> {
    let mut fb: Vec<String> = Vec::new();

    for i in 1..=100 {
        if i % num_one == 0 && i % num_two == 0 {
            fb.push(format!("{string_one}{string_two}"))
        } else if i % num_one == 0 {
            fb.push(string_one.to_string())
        } else if i % num_two == 0 {
            fb.push(string_two.to_string())
        } else {
            fb.push(i.to_string())
        }
    }

    fb
}

#[macro_export]
macro_rules! fizz_buzz_custom {
    () => {
        fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5)
    };
    ($str_one:expr) => {
        fizz_buzz_custom_solver($str_one, "Buzz", 3, 5)
    };
    ($str_one:expr, $str_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, 3, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two)
    };
}

fn main() {
    assert_eq!(fizz_buzz_custom!()[3], "4".to_string());
    assert_eq!(fizz_buzz_custom!()[15], "16".to_string());
}
