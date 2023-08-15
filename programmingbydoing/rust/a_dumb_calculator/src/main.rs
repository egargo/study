use text_io::read;

fn main() {
    print!("What is your first number? ");
    let n1: f32 = read!();

    print!("What is your second number? ");
    let n2: f32 = read!();

    print!("What is your third number? ");
    let n3: f32 = read!();

    let calc = (n1 + n2 + n3) / 2.0;

    println!("( {} + {} + {} ) / 2 is... {}", n1, n2, n3, calc);
}
