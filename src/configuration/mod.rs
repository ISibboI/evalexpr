use crate::{error::Error, value::Value};
use std::collections::HashMap;

pub trait Configuration {
    fn get_value(&self, identifier: &str) -> Option<&Value>;
}

pub struct EmptyConfiguration;

impl Configuration for EmptyConfiguration {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        None
    }
}

pub type HashMapConfiguration = HashMap<String, Value>;

impl Configuration for HashMapConfiguration {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.get(identifier)
    }
}