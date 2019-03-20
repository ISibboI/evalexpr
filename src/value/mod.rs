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

    /// Returns `self` as a `IntType`, or `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> Result<IntType, Error> {
        match self {
            Value::Int(i) => Ok(*i),
            value => Err(Error::expected_int(value.clone())),
        }
    }

    /// Returns `self` as a `FloatType`, or `Err` if `self` is not a `Value::Float` or `Value::Int`.
    pub fn as_float(&self) -> Result<FloatType, Error> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            value => Err(Error::expected_number(value.clone())),
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
