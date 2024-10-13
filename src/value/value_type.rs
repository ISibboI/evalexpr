use crate::Value;

/// The type of a `Value`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ValueType {
    /// The `Value::String` type.
    String,
    /// The `Value::Float` type.
    Float,
    /// The `Value::Int` type.
    Int,
    /// The `Value::Boolean` type.
    Boolean,
    /// The `Value::Tuple` type.
    Tuple,
    /// The `Value::Empty` type.
    Empty,
}

impl<IntType, FloatType> From<&Value<IntType, FloatType>> for ValueType {
    fn from(value: &Value<IntType, FloatType>) -> Self {
        match value {
            Value::String(_) => ValueType::String,
            Value::Float(_) => ValueType::Float,
            Value::Int(_) => ValueType::Int,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Tuple(_) => ValueType::Tuple,
            Value::Empty => ValueType::Empty,
        }
    }
}

impl<IntType, FloatType> From<&mut Value<IntType, FloatType>> for ValueType {
    fn from(value: &mut Value<IntType, FloatType>) -> Self {
        From::<&Value<IntType, FloatType>>::from(value)
    }
}

impl<IntType, FloatType> From<&&mut Value<IntType, FloatType>> for ValueType {
    fn from(value: &&mut Value<IntType, FloatType>) -> Self {
        From::<&Value<IntType, FloatType>>::from(*value)
    }
}
