// https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/rust

macro_rules! octect {
    ($n: expr, $p: expr) => {
        ($n / u32::pow(256, $p) % 256).to_string()
    };
}

fn int32_to_ip(int: u32) -> String {
    (0..=3)
        .into_iter()
        .map(|i| octect!(int, i))
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect::<Vec<String>>()
        .join(".")
}

fn main() {
    assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
    assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
    assert_eq!(int32_to_ip(0), "0.0.0.0");
}
