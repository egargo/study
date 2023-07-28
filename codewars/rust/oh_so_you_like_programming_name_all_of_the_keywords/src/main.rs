// https://www.codewars.com/kata/634ac4e77611b9f57dff456d/train/rust

use std::collections::HashSet;

/* only 39 strict keywords counts, no reserved like abstract or override - as of 2021 Edition */
fn keywords() -> HashSet<&'static str> {
    HashSet::from([
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "union", "unsafe", "use", "where", "while",
    ])
}

/* If you'll find some definitions in tests too obscure or too obvious, don't hesitate to
open an issue with a better replacement! */

fn main() {
    println!("{:?}", keywords().len());
}
