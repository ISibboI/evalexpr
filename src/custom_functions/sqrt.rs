use std::convert::TryInto;
use std::fmt::Debug;
use crate::{Error, Value};
use crate::Error::CustomError;

pub fn sqrt<T: TryInto<Value>>(value: T) -> Result<Value, Error>
where
    <T as TryInto<Value>>::Error: Debug,
{
    // Try converting the value to a Value type
    match value.try_into().map_err(|err| CustomError(format!("{err:?}")))? {
        Value::Float(fl) => {
            if fl < 0.0 {
                Err(Error::InvalidArgumentType)
            } else {
                Ok(Value::Float(fl.sqrt()))
            }
        }
        Value::Int(nn) => {
            if nn < 0 {
                Err(Error::InvalidArgumentType)
            } else {
                Ok(Value::Float((nn as f64).sqrt())) // Convert to float for square root
            }
        }
        Value::Empty => Ok(Value::Empty),
        _ => Err(Error::InvalidArgumentType),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_with_positive_float() {
        let value = Value::Float(4.0);
        let result = sqrt(value).unwrap();
        assert_eq!(result, Value::Float(2.0));
    }

    #[test]
    fn test_sqrt_with_positive_integer() {
        let value = Value::Int(16);
        let result = sqrt(value).unwrap();
        assert_eq!(result, Value::Float(4.0));
    }

    #[test]
    fn test_sqrt_with_zero() {
        let value = Value::Int(0);
        let result = sqrt(value).unwrap();
        assert_eq!(result, Value::Float(0.0));
    }

    #[test]
    fn test_sqrt_with_negative_float() {
        let value = Value::Float(-4.0);
        let result = sqrt(value);
        assert!(result.is_err());
        if let Err(Error::InvalidArgumentType) = result {
            // Test passes
        } else {
            panic!("Expected Error::InvalidArgumentType");
        }
    }

    #[test]
    fn test_sqrt_with_negative_integer() {
        let value = Value::Int(-16);
        let result = sqrt(value);
        assert!(result.is_err());
        if let Err(Error::InvalidArgumentType) = result {
            // Test passes
        } else {
            panic!("Expected Error::InvalidArgumentType");
        }
    }

    #[test]
    fn test_sqrt_with_empty_value() {
        let value = Value::Empty;
        let result = sqrt(value).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_sqrt_invalid_conversion() {
        struct InvalidType;
        let value = Value::String("invalid".to_owned().into());
        let result = sqrt(value);
        assert!(result.is_err());
    }
}

