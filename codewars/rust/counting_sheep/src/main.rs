// https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust


fn count_sheep(sheep: &[bool]) -> u8 {
    sheep
        .iter()
        .filter(|i| **i == true)
        .map(|i| i)
        .collect::<Vec<_>>()
        .iter()
        .count() as u8
}

fn main() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}
