// https://www.codewars.com/kata/577ff15ad648a14b780000e7/train/rust


use std::collections::HashMap;

fn greet(language: &str) -> &str {
    let langs: HashMap<&str, &str> = HashMap::from([
        ("english", "Welcome"),
        ("czech", "Vitejte"),
        ("danish", "Velkomst"),
        ("dutch", "Welkom"),
        ("estonian", "Tere tulemast"),
        ("finnish", "Tervetuloa"),
        ("flemish", "Welgekomen"),
        ("french", "Bienvenue"),
        ("german", "Willkommen"),
        ("irish", "Failte"),
        ("italian", "Benvenuto"),
        ("latvian", "Gaidits"),
        ("lithuanian", "Laukiamas"),
        ("polish", "Witamy"),
        ("spanish", "Bienvenido"),
        ("swedish", "Valkommen"),
        ("welsh", "Croeso"),
    ]);

    langs.get(language).unwrap_or(&"Welcome")
}

fn main() {
    assert_eq!(greet("english"), "Welcome");
    assert_eq!(greet("dutch"), "Welkom");
    assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
    assert_eq!(greet(""), "Welcome");
    assert_eq!(greet("swelsh"), "Welcome");
}
