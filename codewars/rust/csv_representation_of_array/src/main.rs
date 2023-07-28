// https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust

fn to_csv_text(array: &[Vec<i8>]) -> String {
    let mut text = String::new();

    for i in array {
        text.push_str(
            &i.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        text.push_str("\n");
    }

    text[0..text.len() - 1].to_string()
}

fn main() {
    let array = vec![
        vec![0, 1, 2, 3, 4],
        vec![10, 11, 12, 13, 14],
        vec![20, 21, 22, 23, 24],
        vec![30, 31, 32, 33, 34],
    ];
    println!("{:?}", to_csv_text(&array));
}
