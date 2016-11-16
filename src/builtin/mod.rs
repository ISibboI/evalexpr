
use {Function, Functions, Value, to_value};
use math::Math;
use error::Error;


pub struct BuiltIn {}

impl BuiltIn {
    pub fn new() -> Functions {
        let mut functions = Functions::new();
        functions.insert("min".to_owned(), create_min_fuction());
        functions.insert("max".to_owned(), create_max_fuction());
        functions.insert("is_empty".to_owned(), create_is_empty_fuction());
        functions.insert("array".to_owned(), create_array_function());
        functions
    }
}

#[derive(PartialEq)]
enum Compare {
    Min,
    Max
}

fn create_min_fuction() -> Function {
    compare(Compare::Min)
}

fn create_max_fuction() -> Function {
    compare(Compare::Max)
}

fn compare(compare: Compare) -> Function {
    Function {
        max_args: None,
        min_args: Some(1),
        compiled: Box::new(move |values| {
            let mut prev: Result<Value, Error> = Err(Error::Custom("can't find min value.".to_owned()));

            for value in values {
                match value {
                    Value::Array(array) => {
                        for value in array {
                            if prev.is_ok() {
                                if compare == Compare::Min {
                                    if value.lt(prev.as_ref().unwrap())? == to_value(true) {
                                        prev = Ok(value)
                                    }
                                } else {
                                    if value.gt(prev.as_ref().unwrap())? == to_value(true) {
                                        prev = Ok(value)
                                    }
                                }
                            } else {
                                prev = Ok(value);
                            }
                        }
                    },
                    _ => {
                        if prev.is_ok() {
                            if compare == Compare::Min {
                                if value.lt(prev.as_ref().unwrap())? == to_value(true) {
                                    prev = Ok(value)
                                }
                            } else {
                                if value.gt(prev.as_ref().unwrap())? == to_value(true) {
                                    prev = Ok(value)
                                }
                            }
                        } else {
                            prev = Ok(value);
                        }
                    }
                }
            }
            prev
        })
    }
}


fn create_is_empty_fuction() -> Function {
    Function {
        max_args: Some(1),
        min_args: Some(1),
        compiled: Box::new(|values|{
            match *values.first().unwrap() {
                Value::String(ref string) => Ok(Value::Bool(string.is_empty())),
                Value::Array(ref array) => Ok(Value::Bool(array.is_empty())),
                Value::Object(ref object) => Ok(Value::Bool(object.is_empty())),
                Value::Null => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false))
            }
        })
    }
}

fn create_array_function() -> Function {
    Function::new(|values|Ok(to_value(values)))
}
