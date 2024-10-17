use crate::Value;

use super::numeric_types::EvalexprNumericTypes;

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

impl<NumericTypes: EvalexprNumericTypes> From<&Value<NumericTypes>> for ValueType {
    fn from(value: &Value<NumericTypes>) -> Self {
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

impl<NumericTypes: EvalexprNumericTypes> From<&mut Value<NumericTypes>> for ValueType {
    fn from(value: &mut Value<NumericTypes>) -> Self {
        From::<&Value<NumericTypes>>::from(value)
    }
}

impl<NumericTypes: EvalexprNumericTypes> From<&&mut Value<NumericTypes>> for ValueType {
    fn from(value: &&mut Value<NumericTypes>) -> Self {
        From::<&Value<NumericTypes>>::from(*value)
    }
}
