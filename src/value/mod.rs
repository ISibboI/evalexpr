use error::Error;

mod display;

/// The type used to represent integers in `Value::Int`.
pub type IntType = i64;

/// The type used to represent floats in `Value::Float`.
pub type FloatType = f64;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType = Vec<Value>;

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq)]
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
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        match self {
            Value::Int(_) => true,
            _ => false,
        }
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        match self {
            Value::Tuple(_) => true,
            _ => false,
        }
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> Result<String, Error> {
        match self {
            Value::String(string) => Ok(string.clone()),
            value => Err(Error::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> Result<IntType, Error> {
        match self {
            Value::Int(i) => Ok(*i),
            value => Err(Error::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_float(&self) -> Result<FloatType, Error> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            value => Err(Error::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> Result<bool, Error> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(Error::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `TupleType`, or  returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_tuple(&self) -> Result<TupleType, Error> {
        match self {
            Value::Tuple(tuple) => Ok(tuple.clone()),
            value => Err(Error::expected_tuple(value.clone())),
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

impl From<Value> for Result<Value, Error> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use value::{TupleType, Value};

    #[test]
    fn test_value_conversions() {
        assert_eq!(
            Value::from("string").as_string().unwrap(),
            String::from("string")
        );
        assert_eq!(Value::from(3).as_int().unwrap(), 3);
        assert_eq!(Value::from(3.3).as_float().unwrap(), 3.3);
        assert_eq!(Value::from(true).as_boolean().unwrap(), true);
        assert_eq!(
            Value::from(TupleType::new()).as_tuple().unwrap(),
            TupleType::new()
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
}
