use error::Error;

pub type IntType = i64;
pub type FloatType = f64;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Float(FloatType),
    Int(IntType),
    Boolean(bool),
}

impl Value {
    pub fn is_int(&self) -> bool {
        match self {
            Value::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn as_int(&self) -> Result<IntType, Error> {
        match self {
            Value::Int(i) => Ok(*i),
            _ => Err(Error::TypeError),
        }
    }

    pub fn as_float(&self) -> Result<FloatType, Error> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            _ => Err(Error::TypeError),
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

impl From<Value> for Result<Value, Error> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}