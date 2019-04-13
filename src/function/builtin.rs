#[cfg(feature = "regex_support")]
use regex::Regex;

use crate::error::*;
use value::{FloatType, IntType};
use EvalexprError;
use Function;
use Value;

pub fn builtin_function(identifier: &str) -> Option<Function> {
    match identifier {
        "min" => Some(Function::new(
            None,
            Box::new(|arguments| {
                let mut min_int = IntType::max_value();
                let mut min_float = 1.0f64 / 0.0f64;
                debug_assert!(min_float.is_infinite());

                for argument in arguments {
                    if let Value::Float(float) = argument {
                        min_float = min_float.min(*float);
                    } else if let Value::Int(int) = argument {
                        min_int = min_int.min(*int);
                    } else {
                        return Err(EvalexprError::expected_number(argument.clone()));
                    }
                }

                if (min_int as FloatType) < min_float {
                    Ok(Value::Int(min_int))
                } else {
                    Ok(Value::Float(min_float))
                }
            }),
        )),
        "max" => Some(Function::new(
            None,
            Box::new(|arguments| {
                let mut max_int = IntType::min_value();
                let mut max_float = -1.0f64 / 0.0f64;
                debug_assert!(max_float.is_infinite());

                for argument in arguments {
                    if let Value::Float(float) = argument {
                        max_float = max_float.max(*float);
                    } else if let Value::Int(int) = argument {
                        max_int = max_int.max(*int);
                    } else {
                        return Err(EvalexprError::expected_number(argument.clone()));
                    }
                }

                if (max_int as FloatType) > max_float {
                    Ok(Value::Int(max_int))
                } else {
                    Ok(Value::Float(max_float))
                }
            }),
        )),

        "len" => Some(Function::new(
            Some(1),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                Ok(Value::from(subject.len() as i64))
            }),
        )),

        // string functions

        #[cfg(feature = "regex_support")]
        "str::regex_matches" => Some(Function::new(
            Some(2),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                let re_str = expect_string(&arguments[1])?;
                match Regex::new(re_str) {
                    Ok(re) => Ok(Value::Boolean(re.is_match(subject))),
                    Err(err) => Err(EvalexprError::invalid_regex(re_str.to_string(), format!("{}", err)))
                }
            }),
        )),
        #[cfg(feature = "regex_support")]
        "str::regex_replace" => Some(Function::new(
            Some(3),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                let re_str = expect_string(&arguments[1])?;
                let repl = expect_string(&arguments[2])?;
                match Regex::new(re_str) {
                    Ok(re) => Ok(Value::String(re.replace_all(subject, repl).to_string())),
                    Err(err) => Err(EvalexprError::invalid_regex(re_str.to_string(), format!("{}", err))),
                }
            }),
        )),
        "str::to_lowercase" => Some(Function::new(
            Some(1),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                Ok(Value::from(subject.to_lowercase()))
            }),
        )),
        "str::to_uppercase" => Some(Function::new(
            Some(1),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                Ok(Value::from(subject.to_uppercase()))
            }),
        )),
        "str::trim" => Some(Function::new(
            Some(1),
            Box::new(|arguments| {
                let subject = expect_string(&arguments[0])?;
                Ok(Value::from(subject.trim()))
            }),
        )),
        _ => None,
    }
}
