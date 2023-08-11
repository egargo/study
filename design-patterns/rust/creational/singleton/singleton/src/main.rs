#[derive(Debug, PartialEq)]
struct LinuxDistro<'a> {
    name: Option<&'a str>,
    base: Option<&'a str>,
    pkg: Option<Vec<&'a str>>,
    year: Option<u32>,
    active: Option<bool>,
}

impl<'a> LinuxDistro<'a> {
    fn new() -> Self {
        Self {
            name: None,
            base: None,
            pkg: None,
            year: None,
            active: None,
        }
    }

    fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn base(&mut self, base: &'a str) -> &mut Self {
        self.base = Some(base);
        self
    }

    fn pkg(&mut self, pkg: Vec<&'a str>) -> &mut Self {
        self.pkg = Some(pkg);
        self
    }

    fn year(&mut self, year: u32) -> &mut Self {
        self.year = Some(year);
        self
    }

    fn active(&mut self, active: bool) -> &mut Self {
        self.active = Some(active);
        self
    }
}

fn main() {
    let mut distro = LinuxDistro::new();

    distro
        .name("Pop!_OS")
        .base("Ubuntu")
        .pkg(vec!["apt", "dpkg"])
        .year(1993)
        .active(true);

    println!("{:?}", distro);
}
