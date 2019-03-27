use Value;

/// The type of a `Value`.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ValueType {
    String,
    Float,
    Int,
    Boolean,
    Tuple,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(_) => ValueType::String,
            Value::Float(_) => ValueType::Float,
            Value::Int(_) => ValueType::Int,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Tuple(_) => ValueType::Tuple,
        }
    }
}

impl From<&mut Value> for ValueType {
    fn from(value: &mut Value) -> Self {
        From::<&Value>::from(value)
    }
}

impl From<&&mut Value> for ValueType {
    fn from(value: &&mut Value) -> Self {
        From::<&Value>::from(*value)
    }
}