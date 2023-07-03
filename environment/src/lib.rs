use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
    Num(f64),
    Str(String),
    Bool(bool),
    None,
    Function(String),
}

pub struct Environment {
    values: HashMap<String, Type>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Type) {
        self.values.insert(name, value);
    }

    pub fn retrieve(&self, name: &String) -> Option<&Type> {
        self.values.get(name)
    }
}
