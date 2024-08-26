use std::convert::TryInto;
use std::fmt::Debug;
use crate::{Error, Value};
use crate::Error::CustomError;

pub fn pow<T: TryInto<Value> ,E: TryInto<Value>>(base: T, exp: E ) -> Result<Value, Error>
where
    <T as TryInto<Value>>::Error: Debug,
    <E as TryInto<Value>>::Error: Debug,
{
    // Try converting the base and exponent to Value types
    let base = base.try_into().map_err(|err| CustomError(format!("{err:?}")))?;
    let exp = exp.try_into().map_err(|err| CustomError(format!("{err:?}")))?;

    // Match on the base value and compute the power
    match (base, exp) {
        (Value::Float(bf), Value::Float(ef)) => {
            Ok(Value::Float(bf.powf(ef)))
        }
        (Value::Int(bi), Value::Int(ei)) => {
            Ok(Value::Int(bi.pow(ei as u32))) // assuming `ei` fits in a `u32`
        }
        (Value::Float(bf), Value::Int(ei)) => {
            Ok(Value::Float(bf.powi(ei as i32))) // assuming `ei` fits in an `i32`
        }
        (Value::Int(bi), Value::Float(ef)) => {
            Ok(Value::Float((bi as f64).powf(ef)))
        }
        _ => Err(Error::CustomError("Invalidat artument type passed to pow function".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_with_floats() {
        let base = Value::Float(2.0);
        let exp = Value::Float(3.0);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Float(8.0));
    }

    #[test]
    fn test_pow_with_integers() {
        let base = Value::Int(2);
        let exp = Value::Int(3);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Int(8));
    }

    #[test]
    fn test_pow_float_base_integer_exp() {
        let base = Value::Float(2.0);
        let exp = Value::Int(3);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Float(8.0));
    }

    #[test]
    fn test_pow_integer_base_float_exp() {
        let base = Value::Int(2);
        let exp = Value::Float(3.0);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Float(8.0));
    }

    #[test]
    fn test_pow_with_zero_exponent() {
        let base = Value::Int(5);
        let exp = Value::Int(0);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Int(1));
    }

    #[test]
    fn test_pow_with_zero_base() {
        let base = Value::Int(0);
        let exp = Value::Int(5);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_pow_with_negative_exponent() {
        let base = Value::Float(2.0);
        let exp = Value::Int(-3);
        let result = pow(base, exp).unwrap();
        assert_eq!(result, Value::Float(0.125));
    }

    #[test]
    fn test_pow_invalid_argument_type() {
        let base = Value::Empty;
        let exp = Value::Int(2);
        let result = pow(base, exp);
        assert!(result.is_err());
        if let Err(Error::InvalidArgumentType) = result {
            // Test passes
        } else {
            panic!("Expected Error::InvalidArgumentType");
        }
    }

    #[test]
    fn test_pow_invalid_conversion() {
        struct InvalidType;
        let base = Value::String("invalid".to_owned().into());
        let exp = Value::Int(2);
        let result = pow(base, &exp);
        assert!(result.is_err());
    }
}

