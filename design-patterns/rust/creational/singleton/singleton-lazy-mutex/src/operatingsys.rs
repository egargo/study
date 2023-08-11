use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct OperatingSystems {
    pub name: String,
    pub kernel: String,
    pub source: String,
}

impl OperatingSystems {
    pub fn new() -> OperatingSystems {
        OperatingSystems {
            name: String::new(),
            kernel: String::new(),
            source: String::new(),
        }
    }

    pub fn instance() -> &'static Mutex<OperatingSystems> {
        lazy_static! {
            static ref INSTANCE: Mutex<OperatingSystems> = Mutex::new(OperatingSystems::new());
        }

        &INSTANCE
    }

    pub fn os_name(&mut self, name: String, kernel: String, source: String) {
        self.name = name;
        self.kernel = kernel;
        self.source = source;
    }

    pub fn get_os(&self) -> OperatingSystems {
        OperatingSystems {
            name: self.name.clone(),
            kernel: self.kernel.clone(),
            source: self.source.clone(),
        }
    }
}
