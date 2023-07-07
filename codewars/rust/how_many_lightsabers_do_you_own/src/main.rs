// https://www.codewars.com/kata/51f9d93b4095e0a7200001b8/train/rust


fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    match name {
        "Zach" => 18,
        _ => 0,
    }
}

fn main() {
    assert_eq!(how_many_lightsabers_do_you_own(""), 0);
    assert_eq!(how_many_lightsabers_do_you_own("Adam"), 0);
    assert_eq!(how_many_lightsabers_do_you_own("zach"), 0);
    assert_eq!(how_many_lightsabers_do_you_own("Zach"), 18);
}
