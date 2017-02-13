
use serde_json::Value;
use to_value;
use error::Error;

pub trait Math {
    fn add(&self, &Value) -> Result<Value, Error>;
    fn mul(&self, &Value) -> Result<Value, Error>;
    fn sub(&self, &Value) -> Result<Value, Error>;
    fn div(&self, &Value) -> Result<Value, Error>;
    fn rem(&self, &Value) -> Result<Value, Error>;
    fn eq(&self, &Value) -> Result<Value, Error>;
    fn ne(&self, &Value) -> Result<Value, Error>;
    fn gt(&self, &Value) -> Result<Value, Error>;
    fn lt(&self, &Value) -> Result<Value, Error>;
    fn ge(&self, &Value) -> Result<Value, Error>;
    fn le(&self, &Value) -> Result<Value, Error>;
    fn and(&self, &Value) -> Result<Value, Error>;
    fn or(&self, &Value) -> Result<Value, Error>;
}

impl Math for Value {
    fn add(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            if self.is_f64() || value.is_f64() {
                Ok(to_value(self.get_f64() + value.get_f64()))
            } else if self.is_i64() || value.is_i64() {
                Ok(to_value(self.get_i64() + value.get_i64()))
            } else {
                Ok(to_value(self.get_u64() + value.get_u64()))
            }
        } else if self.is_string() && value.is_string() {
            Ok(to_value(self.get_string() + value.get_str()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn mul(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            if self.is_f64() || value.is_f64() {
                Ok(to_value(self.get_f64() * value.get_f64()))
            } else if self.is_i64() || value.is_i64() {
                Ok(to_value(self.get_i64() * value.get_i64()))
            } else {
                Ok(to_value(self.get_u64() * value.get_u64()))
            }
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn sub(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            if self.is_f64() || value.is_f64() {
                Ok(to_value(self.get_f64() - value.get_f64()))
            } else {
                Ok(to_value(self.get_i64() - value.get_i64()))
            }
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn div(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() / value.get_f64()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn rem(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            if self.is_f64() || value.is_f64() {
                Ok(to_value(self.get_f64() % value.get_f64()))
            } else if self.is_i64() || value.is_i64() {
                Ok(to_value(self.get_i64() % value.get_i64()))
            } else {
                Ok(to_value(self.get_u64() % value.get_u64()))
            }
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn eq(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() == value.get_f64()))
        } else {
            Ok(to_value(self == value))
        }
    }

    fn ne(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() != value.get_f64()))
        } else {
            Ok(to_value(self != value))
        }
    }

    fn gt(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() > value.get_f64()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn lt(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() < value.get_f64()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn ge(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() >= value.get_f64()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn le(&self, value: &Value) -> Result<Value, Error> {
        if self.is_number() && value.is_number() {
            Ok(to_value(self.get_f64() <= value.get_f64()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn and(&self, value: &Value) -> Result<Value, Error> {
        if self.is_boolean() && value.is_boolean() {
            Ok(to_value(self.get_boolean() && value.get_boolean()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }

    fn or(&self, value: &Value) -> Result<Value, Error> {
        if self.is_boolean() && value.is_boolean() {
            Ok(to_value(self.get_boolean() || value.get_boolean()))
        } else {
            Err(Error::UnsupportedTypes(self.format(), value.format()))
        }
    }
}


trait Type {
    fn get_f64(&self) -> f64;
    fn get_string(&self) -> String;
    fn get_str(&self) -> &str;
    fn get_u64(&self) -> u64;
    fn get_i64(&self) -> i64;
    fn get_boolean(&self) -> bool;
    fn format(&self) -> String;
}

impl Type for Value {
    fn get_f64(&self) -> f64 {
        match *self {
            Value::Number(ref n) => n.as_f64().unwrap(),
            _ => panic!("not a number"),
        }
    }

    fn get_string(&self) -> String {
        self.as_str().unwrap().to_owned()
    }

    fn get_str(&self) -> &str {
        self.as_str().unwrap()
    }

    fn get_u64(&self) -> u64 {
        self.as_u64().unwrap()
    }

    fn get_i64(&self) -> i64 {
        self.as_i64().unwrap()
    }

    fn get_boolean(&self) -> bool {
        self.as_bool().unwrap()
    }

    fn format(&self) -> String {
        format!("{:?}", self)
    }
}
