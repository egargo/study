#[derive(Clone, Debug)]
struct Linux<'a> {
    name: &'a str,
    pkg: Vec<&'a str>,
    base: Option<&'a str>,
}

fn main() {
    let debian = Linux {
        name: "Debian",
        pkg: vec!["dpkg"],
        base: None,
    };

    let mut ubuntu = debian.clone();
    ubuntu.name = "Ubuntu";
    ubuntu.pkg = vec!["apt", "snap"];
    ubuntu.base = Some(debian.name);

    let mut pop = ubuntu.clone();
    pop.name = "Pop!_OS";
    pop.pkg = vec!["apt", "flatpak"];
    pop.base = Some(ubuntu.name);

    println!("{:?}", debian);
    println!("{:?}", ubuntu);
    println!("{:?}", pop);
}
