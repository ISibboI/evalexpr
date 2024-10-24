use std::convert::TryInto;
use std::fmt::Debug;
use crate::{Error, Value};
use crate::Error::CustomError;

pub fn atan<T: TryInto<Value>>(value: T) -> Result<Value, Error>
where
    <T as TryInto<Value>>::Error: Debug,
{
    // Try converting the value to a Value type
    match value.try_into().map_err(|err| CustomError(format!("{err:?}")))? {
        Value::Float(fl) => {
            Ok(Value::Float(fl.atan())) // Calculate atan for floating-point values
        }
        Value::Int(nn) => {
            Ok(Value::Float((nn as f64).atan())) // Convert int to float and calculate atan
        }
        Value::Empty => Ok(Value::Empty),
        _ => Err(Error::CustomError("Invalid argument type passed to atan function".to_string())),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atan_with_positive_float() {
        let value = Value::Float(1.0);
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Float(1.0f64.atan()));
    }

    #[test]
    fn test_atan_with_positive_integer() {
        let value = Value::Int(1);
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Float((1 as f64).atan()));
    }

    #[test]
    fn test_atan_with_zero() {
        let value = Value::Int(0);
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Float(0.0f64.atan()));
    }

    #[test]
    fn test_atan_with_negative_float() {
        let value = Value::Float(-1.0);
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Float((-1.0f64).atan()));
    }

    #[test]
    fn test_atan_with_negative_integer() {
        let value = Value::Int(-1);
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Float((-1 as f64).atan()));
    }

    #[test]
    fn test_atan_with_empty_value() {
        let value = Value::Empty;
        let result = atan(value).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_atan_invalid_conversion() {
        struct InvalidType;
        let value = Value::String("invalid".to_owned().into());
        let result = atan(value);
        assert!(result.is_err());
    }
}

