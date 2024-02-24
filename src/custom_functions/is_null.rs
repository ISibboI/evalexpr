use crate::Value;

pub fn is_null(value: &Value) -> &Value {
    match value {
        Value::Empty => &Value::Int(0),
        _ => value,
    }
}
