
use {Function, Functions, Value, to_value};
use math::Math;
use error::Error;


pub struct BuiltIn {}

impl BuiltIn {
    pub fn new() -> Functions {
        let mut functions = Functions::new();
        functions.insert("min".to_owned(), create_min_fuction());
        functions.insert("max".to_owned(), create_max_fuction());
        functions.insert("len".to_owned(), create_len_fuction());
        functions.insert("is_empty".to_owned(), create_is_empty_fuction());
        functions.insert("array".to_owned(), create_array_function());
        functions.insert("converge".to_owned(), create_converge_function());
        functions
    }
}

fn expect_number_f64(value: &Value) -> Result<f64, Error> {
    if let Some(number) = value.as_f64() {
        Ok(number)
    } else {
        Err(Error::Custom(format!("Expected number that can be represented as f64. But the given is: {:?}", value)))
    }
}

fn expect_normal_f64(number: f64) -> Result<f64, Error> {
    if number.is_normal() {
        Ok(number)
    } else {
        Err(Error::Custom(format!("Expected normal number. But the given is: {:?}", number)))
    }
}

fn expect_finite_f64(number: f64) -> Result<f64, Error> {
    if number.is_finite() {
        Ok(number)
    } else {
        Err(Error::Custom(format!("Expected finite number. But the given is: {:?}", number)))
    }
}

#[derive(PartialEq)]
enum Compare {
    Min,
    Max,
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
            let mut prev: Result<Value, Error> = Err(Error::Custom("can't find min value."
                .to_owned()));

            for value in values {
                match value {
                    Value::Array(array) => {
                        for value in array {
                            if prev.is_ok() {
                                if compare == Compare::Min {
                                    if value.lt(prev.as_ref().unwrap())? == to_value(true) {
                                        prev = Ok(value)
                                    }
                                } else if value.gt(prev.as_ref().unwrap())? == to_value(true) {
                                    prev = Ok(value)
                                }
                            } else {
                                prev = Ok(value);
                            }
                        }
                    }
                    _ => {
                        if prev.is_ok() {
                            if compare == Compare::Min {
                                if value.lt(prev.as_ref().unwrap())? == to_value(true) {
                                    prev = Ok(value)
                                }
                            } else if value.gt(prev.as_ref().unwrap())? == to_value(true) {
                                prev = Ok(value)
                            }
                        } else {
                            prev = Ok(value);
                        }
                    }
                }
            }
            prev
        }),
    }
}


fn create_is_empty_fuction() -> Function {
    Function {
        max_args: Some(1),
        min_args: Some(1),
        compiled: Box::new(|values| match *values.first().unwrap() {
            Value::String(ref string) => Ok(to_value(string.is_empty())),
            Value::Array(ref array) => Ok(to_value(array.is_empty())),
            Value::Object(ref object) => Ok(to_value(object.is_empty())),
            Value::Null => Ok(to_value(true)),
            _ => Ok(to_value(false)),
        }),
    }
}

fn create_len_fuction() -> Function {
    Function {
        max_args: Some(1),
        min_args: Some(1),
        compiled: Box::new(|values| {
            let value = values.first().unwrap();
            match *value {
                Value::String(ref string) => Ok(to_value(string.len())),
                Value::Array(ref array) => Ok(to_value(array.len())),
                Value::Object(ref object) => Ok(to_value(object.len())),
                Value::Null => Ok(to_value(0)),
                _ => {
                    Err(Error::Custom(format!("len() only accept string, array, object and \
                                               null. But the given is: {:?}",
                                              value)))
                }
            }
        }),
    }
}

fn create_array_function() -> Function {
    Function::new(|values| Ok(to_value(values)))
}

/// Converges exponentially agains `conv_y`, starting from `start_y`.
/// The first four parameters parameterize a function `f`, the last one is the `x`-value where the function should be evaluated.
/// The result of the call is `f(x)`.
///
/// The parameters `start_x` and `start_y` set the starting point of the function.
/// It holds that `f(start_x) = start_y`.
/// The parameter `conv_y` is the convergence target value.
/// It is never reached (assuming no numerical errors).
/// The parameter `step_x` is the "speed" of the convergence, that is how fast the function converges against `conv_y`.
/// In detail, the absolute difference between `start_y` and `conv_y` halves if `x` is increased by `step_x`.
///
/// All parameters are expected to be numbers.
/// The parameters `start_x`, `start_y` and `conv_y` are expected to be finite.
/// The parameter `step_x` is expected to be `normal`.
///
/// # Examples
///
/// The function `2^(-x)` is expressed as `conv(0, 1, 1, 0, x)`.
/// This is the same as `converge(1, 0.5, 1, 0, x)`.
fn create_converge_function() -> Function {
    Function {
        max_args: Some(5),
        min_args: Some(5),
        compiled: Box::new(|values| {
            let start_x = expect_finite_f64(expect_number_f64(&values[0])?)?;
            let start_y = expect_finite_f64(expect_number_f64(&values[1])?)?;
            let step_x = expect_normal_f64(expect_number_f64(&values[2])?)?;
            let conv_y = expect_finite_f64(expect_number_f64(&values[3])?)?;
            let x = expect_number_f64(&values[4])?;

            let units = (x - start_x) / step_x;
            let interpolation_factor = 2.0_f64.powf(-units);
            Ok(to_value(interpolation_factor * start_y + (1.0 - interpolation_factor) * conv_y))
        }),
    }
}

#[cfg(test)]
mod test {
    use crate::{Expr, to_value};

    #[test]
    fn test_conv() {
        let valid_test_cases = vec![
            ("converge(0, 4, 10, 0, 0)", "4.0"),
            ("converge(0, 4, 10, 0, 10)", "2.0"),
            ("converge(0, 4, 10, 0, 5)", "2.8284271247461900976033774484193961571393437507538961"),
            ("converge(0, 4, 10, 0, 20)", "1.0"),
            ("converge(0, 4, 10, 0, 0-10)", "8.0"),
        ];

        for (term, result) in valid_test_cases {
            assert_eq!(Expr::new(format!("({term} < {result} * 1.0001) && ({term} > {result} / 1.0001)", term = term, result = result)).exec(), Ok(to_value(true)));
        }
    }
}