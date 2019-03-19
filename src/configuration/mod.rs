use crate::value::Value;
use function::Function;
use std::collections::HashMap;

pub trait Configuration {
    fn get_value(&self, identifier: &str) -> Option<&Value>;

    fn get_function(&self, identifier: &str) -> Option<&Function>;
}

pub struct EmptyConfiguration;

impl Configuration for EmptyConfiguration {
    fn get_value(&self, _identifier: &str) -> Option<&Value> {
        None
    }

    fn get_function(&self, _identifier: &str) -> Option<&Function> {
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

    pub fn insert_variable<S: Into<String>, V: Into<Value>>(&mut self, identifier: S, value: V) {
        self.variables.insert(identifier.into(), value.into());
    }

    pub fn insert_function<S: Into<String>>(&mut self, identifier: S, function: Function) {
        self.functions.insert(identifier.into(), function);
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
