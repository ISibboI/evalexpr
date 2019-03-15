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

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Float(FloatType),
    Int(IntType),
}