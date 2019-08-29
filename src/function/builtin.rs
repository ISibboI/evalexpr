#[cfg(feature = "regex_support")]
use regex::Regex;


use crate::value::{FloatType, IntType};
use crate::EvalexprError;
use crate::Function;
use crate::Value;

pub fn builtin_function(identifier: &str) -> Option<Function> {
    match identifier {
        "min" => Some(Function::new(Box::new(|argument| {
            let arguments = argument.as_tuple()?;
            let mut min_int = IntType::max_value();
            let mut min_float = 1.0f64 / 0.0f64;
            debug_assert!(min_float.is_infinite());

            for argument in arguments {
                if let Value::Float(float) = argument {
                    min_float = min_float.min(float);
                } else if let Value::Int(int) = argument {
                    min_int = min_int.min(int);
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }

            if (min_int as FloatType) < min_float {
                Ok(Value::Int(min_int))
            } else {
                Ok(Value::Float(min_float))
            }
        }))),
        "max" => Some(Function::new(Box::new(|argument| {
            let arguments = argument.as_tuple()?;
            let mut max_int = IntType::min_value();
            let mut max_float = -1.0f64 / 0.0f64;
            debug_assert!(max_float.is_infinite());

            for argument in arguments {
                if let Value::Float(float) = argument {
                    max_float = max_float.max(float);
                } else if let Value::Int(int) = argument {
                    max_int = max_int.max(int);
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }

            if (max_int as FloatType) > max_float {
                Ok(Value::Int(max_int))
            } else {
                Ok(Value::Float(max_float))
            }
        }))),

        "len" => Some(Function::new(Box::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.len() as i64))
        }))),

        // string functions
        #[cfg(feature = "regex_support")]
        "str::regex_matches" => Some(Function::new(Box::new(|argument| {
            let arguments = argument.as_tuple()?;

            let subject = arguments[0].as_string()?;
            let re_str = arguments[1].as_string()?;
            match Regex::new(&re_str) {
                Ok(re) => Ok(Value::Boolean(re.is_match(&subject))),
                Err(err) => Err(EvalexprError::invalid_regex(
                    re_str.to_string(),
                    format!("{}", err),
                )),
            }
        }))),
        #[cfg(feature = "regex_support")]
        "str::regex_replace" => Some(Function::new(Box::new(|argument| {
            let arguments = argument.as_tuple()?;

            let subject = arguments[0].as_string()?;
            let re_str = arguments[1].as_string()?;
            let repl = arguments[2].as_string()?;
            match Regex::new(&re_str) {
                Ok(re) => Ok(Value::String(re.replace_all(&subject, repl.as_str()).to_string())),
                Err(err) => Err(EvalexprError::invalid_regex(
                    re_str.to_string(),
                    format!("{}", err),
                )),
            }
        }))),
        "str::to_lowercase" => Some(Function::new(Box::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_lowercase()))
        }))),
        "str::to_uppercase" => Some(Function::new(Box::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_uppercase()))
        }))),
        "str::trim" => Some(Function::new(Box::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.trim()))
        }))),
        _ => None,
    }
}
