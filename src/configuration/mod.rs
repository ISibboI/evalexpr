use crate::value::Value;
use function::Function;
use std::collections::HashMap;

/// A configuration for an expression tree.
///
/// A configuration defines methods to retrieve values and functions for literals in an expression tree.
/// This crate implements two basic variants, the `EmptyConfiguration`, that returns `None` for each identifier, and the `HashMapConfiguration`, that stores its mappings in hash maps.
pub trait Configuration {
    /// Returns the value that is linked to the given identifier.
    fn get_value(&self, identifier: &str) -> Option<&Value>;

    /// Returns the function that is linked to the given identifier.
    fn get_function(&self, identifier: &str) -> Option<&Function>;
}

/// A configuration that returns `None` for each identifier.
pub struct EmptyConfiguration;

impl Configuration for EmptyConfiguration {
    fn get_value(&self, _identifier: &str) -> Option<&Value> {
        None
    }

    fn get_function(&self, _identifier: &str) -> Option<&Function> {
        None
    }
}

/// A configuration that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
pub struct HashMapConfiguration {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl HashMapConfiguration {
    /// Constructs a `HashMapConfiguration` with no mappings.
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
            functions: Default::default(),
        }
    }

    /// Adds a variable mapping to the configuration.
    ///
    /// *Value and function mappings are stored independently, meaning that there can be a function and a variable with the same identifier.*
    pub fn insert_variable<S: Into<String>, V: Into<Value>>(&mut self, identifier: S, value: V) {
        self.variables.insert(identifier.into(), value.into());
    }

    /// Adds a function mappign to the configuration.
    ///
    /// *Value and function mappings are stored independently, meaning that there can be a function and a variable with the same identifier.*
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
