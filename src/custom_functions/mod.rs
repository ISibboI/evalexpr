pub mod triangular_moving_average;
pub mod expression_functions;
pub mod simple_moving_average;
pub mod simple_cumulative_sum;
pub mod back;
pub mod basher_trade_model;
pub mod compiled_transpose_calcuation_template;


// Re-export the functions to the root of the crate
pub use triangular_moving_average::triangular_moving_average;
pub use expression_functions::*;
pub use back::*;
pub use compiled_transpose_calcuation_template::*;
pub use simple_moving_average::simple_moving_average;
pub use simple_cumulative_sum::simple_cumulative_sum;
use crate::Value;

pub fn generate_column_name(field: &str, p1: &Value) -> String {
    format!("{}_{}", field, get_string(p1))
}

pub fn get_string(value: &Value) -> String {
    match value {
        Value::String(v) => {format!("{}", v)}
        Value::Float(v)  => {format!("{}", v)}
        Value::Int(v) => {format!("{}", v)}
        Value::Boolean(v)  => {format!("{}", v)}
        Value::Tuple(v)  => {format!("{:?}", v)}
        Value::Empty  => {"null".to_owned()}
    }
}
