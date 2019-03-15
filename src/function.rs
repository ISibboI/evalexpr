use error::Error;
use value::Value;

pub struct Function {
    parameter_amount: usize,
    function: fn() -> Result<Value, Error>, // TODO continue type
}