// https://www.codewars.com/kata/51c89385ee245d7ddf000001/train/rust


fn solution(n: u32) -> String {
    match n.to_string().len() {
        1 => return format!("Value is 0000{}", n).to_string(),
        2 => return format!("Value is 000{}", n).to_string(),
        3 => return format!("Value is 00{}", n).to_string(),
        4 => return format!("Value is 0{}", n).to_string(),
        5 => return format!("Value is {}", n).to_string(),
        _ => panic!(),
    }
}

fn main() {
    assert_eq!(solution(5), "Value is 00005");
    assert_eq!(solution(1204), "Value is 01204");
    assert_eq!(solution(109), "Value is 00109");
    assert_eq!(solution(0), "Value is 00000");
}
