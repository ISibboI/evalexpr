use crate::{error::Error, value::Value};

pub trait Configuration {
    fn get_value(&self, identifier: &str) -> Result<Value, Error>;
}

pub struct EmptyConfiguration;

impl Configuration for EmptyConfiguration {
    fn get_value(&self, _identifier: &str) -> Result<Value, Error> {
        Err(Error::IdentifierNotFound)
    }
}
