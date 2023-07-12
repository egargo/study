#[derive(Debug)]
struct Linux {
    name: String,
    base: Option<String>,
    desktop: Option<String>,
    active: bool,
}

#[derive(Debug, Clone)]
struct LinuxBuilder {
    name: String,
    base: Option<String>,
    desktop: Option<String>,
    active: bool,
}

impl Linux {
    fn new(name: String) -> LinuxBuilder {
        LinuxBuilder {
            name,
            base: None,
            desktop: None,
            active: true,
        }
    }
}

impl LinuxBuilder {
    fn base(&mut self, base: String) -> &mut Self {
        self.base = Some(base);
        self
    }

    fn desktop(&mut self, desktop: String) -> &mut Self {
        self.desktop = Some(desktop);
        self
    }

    fn build(&mut self) -> Linux {
        Linux {
            name: self.name.to_owned(),
            base: self.base.to_owned(),
            desktop: self.desktop.to_owned(),
            active: self.active.to_owned(),
        }
    }
}

fn main() {
    let pop = Linux::new("Pop!_OS".to_owned())
        .base("Ubuntu".to_owned())
        .desktop("GNOME".to_owned())
        .build();

    println!("{:?}", pop);

}
