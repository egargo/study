use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Countries {
    pub name: String,
    pub capital: String,
    pub population: u32,
}

impl Countries {
    pub fn new() -> Countries {
        Countries {
            name: String::new(),
            capital: String::new(),
            population: 0,
        }
    }

    pub fn instance() -> &'static Mutex<Countries> {
        lazy_static! {
            static ref INSTANCE: Mutex<Countries> = Mutex::new(Countries::new());
        }

        &INSTANCE
    }

    pub fn country_name(&mut self, name: String, capital: String, population: u32) {
        self.name = name;
        self.capital = capital;
        self.population = population;
    }

    pub fn get_country(&self) -> Countries {
        Countries {
            name: self.name.clone(),
            capital: self.capital.clone(),
            population: *&self.population,
        }
    }
}
