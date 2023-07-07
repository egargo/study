// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust


fn abbrev_name(name: &str) -> String {
    let n = name.split(" ").collect::<Vec<_>>();

    format!(
        "{}.{}",
        n[0].chars().nth(0).unwrap(),
        n[1].chars().nth(0).unwrap(),
    )
    .to_uppercase()
}

fn main() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}
