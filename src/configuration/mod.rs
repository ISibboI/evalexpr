use crate::{error::Error, value::Value};
use std::collections::HashMap;
use function::Function;

pub trait Configuration {
    fn get_value(&self, identifier: &str) -> Option<&Value>;

    fn get_function(&self, identifier: &str) -> Option<&Function>;
}

pub struct EmptyConfiguration;

impl Configuration for EmptyConfiguration {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        None
    }

    fn get_function(&self, identifier: &str) -> Option<&Function> {
        None
    }
}

pub struct HashMapConfiguration {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl HashMapConfiguration {
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
            functions: Default::default(),
        }
    }

    pub fn insert_variable(&mut self, identifier: String, value: Value) {
        self.variables.insert(identifier, value);
    }

    pub fn insert_function(&mut self, identifier: String, function: Function) {
        self.functions.insert(identifier, function);
    }
}

impl Configuration for HashMapConfiguration {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.variables.get(identifier)
    }

    fn get_function(&self, identifier: &str) -> Option<&Function> {
        self.functions.get(identifier)
    }
}