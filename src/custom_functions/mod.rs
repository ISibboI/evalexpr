pub mod triangular_moving_average;
pub mod expression_functions;
pub mod simple_moving_average;
pub mod simple_cumulative_sum;
pub mod back;
pub  mod simple_cumilative_returns;
mod rolling_stdev;
mod rolling_min;
pub  mod pow;
mod sqrt;
mod atan;
pub  mod bucket_functions;
pub  mod report_uri_functions;

use std::fmt::Display;
// Re-export the functions to the root of the crate
pub use triangular_moving_average::triangular_moving_average;
pub use triangular_moving_average::columns_len;
pub use expression_functions::*;
pub use back::*;
pub use pow::*;
pub use sqrt::*;
pub use atan::*;
pub use bucket_functions::*;
pub use simple_moving_average::simple_moving_average;
pub use simple_cumulative_sum::simple_cumulative_sum;
pub use simple_cumilative_returns::simple_cumulative_returns;
pub use rolling_stdev::rolling_stdev;
pub use rolling_min::rolling_min;
pub use report_uri_functions::*; 
use crate::Value;

pub fn generate_column_name(field: &str, p1: &Value) -> String {
    format!("{}__{}", field.to_string(), sanitize_with_char(&get_string(p1), 'x'))
}

pub fn sanitize_with_char<T: Display>(value: &T, ch: char) -> String {
    let mut sanitized = String::new();
    for c in format!("{}", value).chars() {
        if c.is_ascii_alphanumeric() {
            sanitized.push(c);
        } else {
            sanitized.push(ch);
        }
    }
    sanitized.replace(&format!("{ch}{ch}"), &format!("{ch}"))
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
