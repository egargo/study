use text_io::read;

fn main() {
    print!("Hello.  What is your name? ");
    let name: String = read!();

    print!("Hi, {name}!  How old are you? ");
    let age: u8 = read!();

    println!(
        "Did you know that in five years you will be {} years old?",
        age + 5
    );

    println!("And five years ago you were {}! Imagine that!", age - 5);

    println!("Hello, world!");
}
