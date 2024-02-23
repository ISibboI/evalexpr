use crate::error::{EvalexprError, EvalexprResult};

mod display;
pub mod value_type;

/// The type used to represent integers in `Value::Int`.
pub type IntType = i64;

/// The type used to represent floats in `Value::Float`.
pub type FloatType = f64;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType = Vec<Value>;

/// The type used to represent empty values in `Value::Empty`.
pub type EmptyType = ();

/// The value of the empty type to be used in rust.
pub const EMPTY_VALUE: () = ();

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Value {
    /// A string value.
    String(String),
    /// A float value.
    Float(FloatType),
    /// An integer value.
    Int(IntType),
    /// A boolean value.
    Boolean(bool),
    /// A tuple value.
    Tuple(TupleType),
    /// An empty value.
    Empty,
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        matches!(self, Value::Tuple(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> EvalexprResult<String> {
        match self {
            Value::String(string) => Ok(string.clone()),
            value => Err(EvalexprError::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> EvalexprResult<IntType> {
        match self {
            Value::Int(i) => Ok(*i),
            value => Err(EvalexprError::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float(&self) -> EvalexprResult<FloatType> {
        match self {
            Value::Float(f) => Ok(*f),
            value => Err(EvalexprError::expected_float(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> EvalexprResult<FloatType> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            value => Err(EvalexprError::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> EvalexprResult<bool> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(EvalexprError::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_tuple(&self) -> EvalexprResult<TupleType> {
        match self {
            Value::Tuple(tuple) => Ok(tuple.clone()),
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_tuple(&self, len: usize) -> EvalexprResult<TupleType> {
        match self {
            Value::Tuple(tuple) => {
                if tuple.len() == len {
                    Ok(tuple.clone())
                } else {
                    Err(EvalexprError::expected_fixed_len_tuple(len, self.clone()))
                }
            },
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_empty(&self) -> EvalexprResult<()> {
        match self {
            Value::Empty => Ok(()),
            value => Err(EvalexprError::expected_empty(value.clone())),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

impl From<FloatType> for Value {
    fn from(float: FloatType) -> Self {
        Value::Float(float)
    }
}

impl From<IntType> for Value {
    fn from(int: IntType) -> Self {
        Value::Int(int)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl From<TupleType> for Value {
    fn from(tuple: TupleType) -> Self {
        Value::Tuple(tuple)
    }
}

impl From<Value> for EvalexprResult<Value> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

use std::ops::{Div, Rem};

use std::ops::Mul;

#[derive(Debug, Clone)]
pub enum Error {
    UnsupportedOperation,
    DivisionByZero,
}

pub trait ToErrorType {
    fn to_error_code(&self) -> i32;
}

impl ToErrorType for Error {
    fn to_error_code(&self) -> i32 {
        match self {
            Error::UnsupportedOperation => 1,
            Error::DivisionByZero => 2,
        }
    }
}

use std::ops::Sub;

use std::ops::Add;

use std::ops::Neg;

impl Neg for Value {
    type Output = Result<Self, Error>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}




impl Rem for &Value {
    type Output = Result<Value, Error>;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(*a % *b as f64)),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Add for &Value {
    type Output = Result<Value, Error>; // Assuming you have an error type defined

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => Ok(Value::Float(*a as FloatType + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            // Handle combinations with strings and numeric types if desired
            (Value::Int(a), Value::String(b)) | (Value::String(b), Value::Int(a)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Float(a), Value::String(b)) | (Value::String(b), Value::Float(a)) => Ok(Value::String(format!("{}{}", a, b))),
            // Add cases for other Value variants as necessary
            _ => Err(Error::UnsupportedOperation),
        }
    }
}


impl Sub for &Value {
    type Output = Result<Value, Error>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(*a - *b as f64)),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Mul for &Value {
    type Output = Result<Value, Error>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => Ok(Value::Float(*a as f64 * b)),
            _ => Err(Error::UnsupportedOperation),
        }
    }
}

impl Div for &Value {
    type Output = Result<Value, Error>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if b == &0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Float(a), Value::Float(b)) => {
                if b == &0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(a / b))
                }
            },
            (Value::Int(a), Value::Float(b)) => {
                if b == &0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            },
            (Value::Float(a), Value::Int(b)) => {
                if b == &0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(*a / *b as f64))
                }
            },
            // Add cases for other combinations as needed, returning UnsupportedOperation for non-numeric types
            _ => Err(Error::UnsupportedOperation),
        }
    }
}



#[repr(C)]
pub struct FfiResult<T> {
    /// The value, which will be a default value in case of an error.
    pub value: T,
    /// An integer error code. 0 indicates success, non-zero indicates an error.
    pub error_code: i32,
}

/// Converts a Rust `Result<T, i32>` to an `FfiResult<T>`, where `T: Default`.
pub fn to_ffi_result<T: Default, E: ToErrorType>(result: Result<T, E>) -> FfiResult<T> {
    match result {
        Ok(value) => FfiResult {
            value,
            error_code: 0, // Indicate success
        },
        Err(e) => FfiResult {
            value: T::default(),
            error_code : e.to_error_code(), // Use the provided error code
        },
    }
}





macro_rules! declare_arithmetic_for_result {
    ($trait:ident, $fn:ident) => {
        impl std::ops::$trait<Value> for Result<Value, Error> {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Value) -> Self::Output {
                match self {
                    Ok(ref self_val) => self_val.$fn(&other),
                    Err(e) => Err(e),
                }
            }
        }

        impl std::ops::$trait<Result<Value, Error>> for Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Result<Value, Error>) -> Self::Output {
                match other {
                    Ok(ref other_val) => (&self).$fn(other_val),
                    Err(e) => Err(e),
                }
            }
        }

        impl std::ops::$trait for Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Self) -> Self::Output {
                (&self).$fn(&other)
            }
        }

        impl std::ops::$trait<&Value> for Value {
            type Output = Result<Value, Error>;
            fn $fn(self, other: &Self) -> Self::Output {
                 (&self).$fn(other)
            }
        }

        impl std::ops::$trait<Value> for &Value {
            type Output = Result<Value, Error>;
            fn $fn(self, other: Value) -> Self::Output {
                 self.$fn(&other)
            }
        }

        impl std::ops::$trait<Result<Value, Error>> for &Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Result<Value, Error>) -> Self::Output {
                match other {
                    Ok(ref other_val) => self.$fn(other_val),
                    Err(e) => Err(e),
                }
            }
        }

    };
}



declare_arithmetic_for_result!(Rem, rem);
declare_arithmetic_for_result!(Add, add);
declare_arithmetic_for_result!(Sub, sub);
declare_arithmetic_for_result!(Mul, mul);
declare_arithmetic_for_result!(Div, div);


#[cfg(test)]
mod tests {
    use crate::value::{TupleType, Value};
    use super::*;
    #[test]
    fn test_value_conversions() {
        assert_eq!(
            Value::from("string").as_string(),
            Ok(String::from("string"))
        );
        assert_eq!(Value::from(3).as_int(), Ok(3));
        assert_eq!(Value::from(3.3).as_float(), Ok(3.3));
        assert_eq!(Value::from(true).as_boolean(), Ok(true));
        assert_eq!(
            Value::from(TupleType::new()).as_tuple(),
            Ok(TupleType::new())
        );
    }

    #[test]
    fn test_value_checks() {
        assert!(Value::from("string").is_string());
        assert!(Value::from(3).is_int());
        assert!(Value::from(3.3).is_float());
        assert!(Value::from(true).is_boolean());
        assert!(Value::from(TupleType::new()).is_tuple());
    }

    #[test]
    fn test_add_integers() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        // Unwrap the result to compare the value directly
        assert_eq!(a.add(b).unwrap(), Value::Int(30));
    }

    #[test]
    fn test_add_integers_to_add() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        let c = Value::Int(20);
        // Unwrap the result to compare the value directly
        assert_eq!(a.add(b).add(c).unwrap(), Value::Int(50));
    }

    #[test]
    fn test_subtract_floats() {
        let a = Value::Float(20.5);
        let b = Value::Float(10.25);
        // Unwrap the result to compare the value directly
        assert_eq!(a.sub(b).unwrap(), Value::Float(10.25));
    }

    #[test]
    fn test_multiply_int_float() {
        let a = Value::Int(2);
        let b = Value::Float(3.5);
        // Unwrap the result to compare the value directly
        assert_eq!(a.mul(b).unwrap(), Value::Float(7.0));
    }

    #[test]
    fn test_divide_float_by_int() {
        let a = Value::Float(10.0);
        let b = Value::Int(2);
        // Unwrap the result to compare the value directly
        assert_eq!(a.div(b).unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_integer_remainder() {
        let a = Value::Int(10);
        let b = Value::Int(4);
        // Unwrap the result to compare the value directly
        assert_eq!(a.rem(b).unwrap(), Value::Int(2));
    }

    #[test]
    fn test_error_on_divide_by_zero() {
        let a = Value::Int(10);
        let b = Value::Int(0);
        // Here, we expect an error, so no unwrap is needed
        assert!(matches!(a.div(b), Err(Error::DivisionByZero)));
    }


        #[test]
        fn test_add_integers_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(20);
            // Using references in the add operation
            assert_eq!((&a).add(&b).unwrap(), Value::Int(30));
        }

        #[test]
        fn test_add_integers_to_add_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(20);
            let c = Value::Int(20);
            // Using references in chained add operations
            assert_eq!((&a).add(&b).unwrap().add(&c).unwrap(), Value::Int(50));
        }

        #[test]
        fn test_subtract_floats_with_refs() {
            let a = Value::Float(20.5);
            let b = Value::Float(10.25);
            // Using references in the sub operation
            assert_eq!((&a).sub(&b).unwrap(), Value::Float(10.25));
        }

        #[test]
        fn test_multiply_int_float_with_refs() {
            let a = Value::Int(2);
            let b = Value::Float(3.5);
            // Using references in the mul operation
            assert_eq!((&a).mul(&b).unwrap(), Value::Float(7.0));
        }

        #[test]
        fn test_divide_float_by_int_with_refs() {
            let a = Value::Float(10.0);
            let b = Value::Int(2);
            // Using references in the div operation
            assert_eq!((&a).div(&b).unwrap(), Value::Float(5.0));
        }

        #[test]
        fn test_integer_remainder_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(4);
            // Using references in the rem operation
            assert_eq!((&a).rem(&b).unwrap(), Value::Int(2));
        }

        #[test]
        fn test_error_on_divide_by_zero_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(0);
            // Using references, expecting an error on division by zero
            assert!(matches!((&a).div(&b), Err(Error::DivisionByZero)));
        }

    #[test]
    fn test_add_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        // Reference on the left, value on the right
        assert_eq!((&a).add(b.clone()).unwrap(), Value::Int(30));
        // Value on the left, reference on the right
        assert_eq!(a.add(&b).unwrap(), Value::Int(30));
    }

    #[test]
    fn test_subtract_ref_and_value() {
        let a = Value::Float(20.5);
        let b = Value::Float(10.25);
        // Reference on the left, value on the right
        assert_eq!((&a).sub(b.clone()).unwrap(), Value::Float(10.25));
        // Value on the left, reference on the right
        assert_eq!(a.sub(&b).unwrap(), Value::Float(10.25));
    }

    #[test]
    fn test_multiply_ref_and_value() {
        let a = Value::Int(2);
        let b = Value::Float(3.5);
        // Reference on the left, value on the right
        assert_eq!((&a).mul(b.clone()).unwrap(), Value::Float(7.0));
        // Value on the left, reference on the right
        assert_eq!(a.mul(&b).unwrap(), Value::Float(7.0));
    }

    #[test]
    fn test_divide_ref_and_value() {
        let a = Value::Float(10.0);
        let b = Value::Int(2);
        // Reference on the left, value on the right
        assert_eq!((&a).div(b.clone()).unwrap(), Value::Float(5.0));
        // Value on the left, reference on the right
        assert_eq!(a.div(&b).unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_remainder_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(4);
        // Reference on the left, value on the right
        assert_eq!((&a).rem(b.clone()).unwrap(), Value::Int(2));
        // Value on the left, reference on the right
        assert_eq!(a.rem(&b).unwrap(), Value::Int(2));
    }

    #[test]
    fn test_error_on_divide_by_zero_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(0);
        // Reference on the left, value on the right
        assert!(matches!((&a).div(b.clone()), Err(Error::DivisionByZero)));
        // Value on the left, reference on the right
        assert!(matches!(a.div(&b), Err(Error::DivisionByZero)));
    }

    }

