// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust

fn accum(s: &str) -> String {
    let s = s.trim().split("").collect::<Vec<_>>();

    let mut rep = 0;
    let mut mumble = String::new();

    for c in &s {
        if !c.is_empty() {
            mumble.push_str(&c.to_uppercase().to_string());
            for _ in 1..rep {
                mumble.push_str(&c.to_lowercase());
            }
            mumble.push_str("-");
        }
        rep += 1;
    }

    mumble[0..mumble.len() - 1].to_string()
}

fn main() {
    println!("{}", accum("RqaEzty"));
}
