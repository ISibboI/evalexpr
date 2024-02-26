use crate::Value;

pub fn is_null(value: &Value) -> &Value {
    match value {
        Value::Empty => &Value::Int(0),
        _ => value,
    }
}
pub fn max<'a>(value1: &'a Value, value2: &'a Value) -> &'a Value {
    if value1 > value2 {
        value1
    } else {
        value2
    }
}

pub fn min<'a>(value1: &'a Value, value2: &'a Value) -> &'a Value {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}
