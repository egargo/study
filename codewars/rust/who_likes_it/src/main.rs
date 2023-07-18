// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust


fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => String::from("no one likes this"),
        1 => format!("{} likes this", names.get(0).unwrap()),
        2 => format!(
            "{} and {} like this",
            names.get(0).unwrap(),
            names.get(1).unwrap()
        ),
        3 => format!(
            "{}, {} and {} like this",
            names.get(0).unwrap(),
            names.get(1).unwrap(),
            names.get(2).unwrap()
        ),
        _ => format!(
            "{}, {} and {} others like this",
            names.get(0).unwrap(),
            names.get(1).unwrap(),
            names.len() - 2
        ),
    }
}

fn main() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
