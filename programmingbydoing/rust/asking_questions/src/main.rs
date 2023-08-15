use text_io::read;

fn main() {
    print!("How old are you? ");
    let age: u8 = read!();

    print!("How many feet tall are you? ");
    let height_ft: u8 = read!();

    print!("And how many inches? ");
    let height_inch: u8 = read!();

    print!("How much do you weigh? ");
    let weight: u8 = read!();

    println!(
        "{}",
        format!(
            "So, you're {} old, {}'{}\" tall and {} heavy.",
            age, height_ft, height_inch, weight
        )
    );
}
