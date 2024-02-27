use crate::Value;

pub fn is_null(value: &Value) -> &Value {
    match value {
        Value::Empty => &Value::Int(0),
        _ => value,
    }
}

pub fn starts_with(message: &Value, prefix: &Value) -> Value {
    if let (Value::String(message), Value::String(prefix)) = (message, prefix) {
        if message.starts_with(prefix) {
            return Value::Boolean(true);
        }
    }
    return Value::Boolean(false);
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
