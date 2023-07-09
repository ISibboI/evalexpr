#[cfg(feature = "regex_support")]
use regex::Regex;

use crate::{
    value::{FloatType, IntType},
    EvalexprError, Function, Value, ValueType,
};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

macro_rules! simple_math {
    ($func:ident) => {
        Some(Function::new(|argument| {
            let num = argument.as_number()?;
            Ok(Value::Float(num.$func()))
        }))
    };
    ($func:ident, 2) => {
        Some(Function::new(|argument| {
            let tuple = argument.as_fixed_len_tuple(2)?;
            let (a, b) = (tuple[0].as_number()?, tuple[1].as_number()?);
            Ok(Value::Float(a.$func(b)))
        }))
    };
}

#[allow(unused)]
fn float_is(func: fn(FloatType) -> bool) -> Option<Function> {
    Some(Function::new(move |argument| {
        Ok(func(argument.as_number()?).into())
    }))
}

macro_rules! int_function {
    ($func:ident) => {
        Some(Function::new(|argument| {
            let int = argument.as_int()?;
            Ok(Value::Int(int.$func()))
        }))
    };
    ($func:ident, 2) => {
        Some(Function::new(|argument| {
            let tuple = argument.as_fixed_len_tuple(2)?;
            let (a, b) = (tuple[0].as_int()?, tuple[1].as_int()?);
            Ok(Value::Int(a.$func(b)))
        }))
    };
}

pub fn builtin_function(identifier: &str) -> Option<Function> {
    #[cfg(feature = "decimal_support")]
    use rust_decimal::prelude::*;
    match identifier {
        // Log
        "math::ln" => simple_math!(ln),
        #[cfg(not(feature = "decimal_support"))]
        "math::log" => simple_math!(log, 2),
        #[cfg(not(feature = "decimal_support"))]
        "math::log2" => simple_math!(log2),
        "math::log10" => simple_math!(log10),
        // Exp
        "math::exp" => simple_math!(exp),
        #[cfg(not(feature = "decimal_support"))]
        "math::exp2" => simple_math!(exp2),
        // Pow
        #[cfg(feature = "decimal_support")]
        "math::pow" => Some(Function::new(|argument| {
            let tuple = argument.as_fixed_len_tuple(2)?;
            let (a, b) = (tuple[0].as_number()?, tuple[1].as_number()?);
            Ok(Value::Float(a.powf(b.to_f64().unwrap())))
        })),
        #[cfg(not(feature = "decimal_support"))]
        "math::pow" => simple_math!(powf, 2),
        // Cos
        "math::cos" => simple_math!(cos),
        #[cfg(not(feature = "decimal_support"))]
        "math::acos" => simple_math!(acos),
        #[cfg(not(feature = "decimal_support"))]
        "math::cosh" => simple_math!(cosh),
        #[cfg(not(feature = "decimal_support"))]
        "math::acosh" => simple_math!(acosh),
        // Sin
        "math::sin" => simple_math!(sin),
        #[cfg(not(feature = "decimal_support"))]
        "math::asin" => simple_math!(asin),
        #[cfg(not(feature = "decimal_support"))]
        "math::sinh" => simple_math!(sinh),
        #[cfg(not(feature = "decimal_support"))]
        "math::asinh" => simple_math!(asinh),
        // Tan
        "math::tan" => simple_math!(tan),
        #[cfg(not(feature = "decimal_support"))]
        "math::atan" => simple_math!(atan),
        #[cfg(not(feature = "decimal_support"))]
        "math::tanh" => simple_math!(tanh),
        #[cfg(not(feature = "decimal_support"))]
        "math::atanh" => simple_math!(atanh),
        #[cfg(not(feature = "decimal_support"))]
        "math::atan2" => simple_math!(atan2, 2),
        // Root
        #[cfg(feature = "decimal_support")]
        "math::sqrt" => Some(Function::new(|argument| {
            let num = argument.as_number()?;
            Ok(Value::Float(num.sqrt().unwrap()))
        })),
        #[cfg(not(feature = "decimal_support"))]
        "math::sqrt" => simple_math!(sqrt),
        #[cfg(not(feature = "decimal_support"))]
        "math::cbrt" => simple_math!(cbrt),
        // Hypotenuse
        #[cfg(not(feature = "decimal_support"))]
        "math::hypot" => simple_math!(hypot, 2),
        // Rounding
        "floor" => simple_math!(floor),
        #[cfg(feature = "decimal_support")]
        "round" => Some(Function::new(|argument| {
            let num = argument.as_number()?;
            Ok(Value::Float(num.round_dp_with_strategy(
                0,
                RoundingStrategy::MidpointAwayFromZero,
            )))
        })),
        #[cfg(not(feature = "decimal_support"))]
        "round" => simple_math!(round),
        "ceil" => simple_math!(ceil),
        // Float special values
        #[cfg(not(feature = "decimal_support"))]
        "math::is_nan" => float_is(FloatType::is_nan),
        #[cfg(not(feature = "decimal_support"))]
        "math::is_finite" => float_is(FloatType::is_finite),
        #[cfg(not(feature = "decimal_support"))]
        "math::is_infinite" => float_is(FloatType::is_infinite),
        #[cfg(not(feature = "decimal_support"))]
        "math::is_normal" => float_is(FloatType::is_normal),
        // Absolute
        "math::abs" => Some(Function::new(|argument| match argument {
            Value::Float(num) => Ok(Value::Float(num.abs())),
            Value::Int(num) => Ok(Value::Int(num.abs())),
            _ => Err(EvalexprError::expected_number(argument.clone())),
        })),
        // Other
        "typeof" => Some(Function::new(move |argument| {
            Ok(match argument {
                Value::String(_) => "string",
                Value::Float(_) => "float",
                Value::Int(_) => "int",
                Value::Boolean(_) => "boolean",
                Value::Tuple(_) => "tuple",
                Value::Empty => "empty",
            }
            .into())
        })),
        "min" => Some(Function::new(|argument| {
            let arguments = argument.as_tuple()?;
            let mut min_int = IntType::max_value();
            let mut min_float = Option::<FloatType>::None;

            for argument in arguments {
                if let Value::Float(float) = argument {
                    if let Some(min) = min_float {
                        min_float = Some(min.max(float));
                    } else {
                        min_float = Some(float);
                    }
                } else if let Value::Int(int) = argument {
                    min_int = min_int.min(int);
                } else {
                    return Err(EvalexprError::expected_number(argument));
                }
            }

            if let Some(min_float) = min_float {
                if ({
                    #[cfg(feature = "decimal_support")]
                    {
                        FloatType::from_i64(min_int).unwrap()
                    }
                    #[cfg(not(feature = "decimal_support"))]
                    {
                        min_int as FloatType
                    }
                }) < min_float
                {
                    Ok(Value::Int(min_int))
                } else {
                    Ok(Value::Float(min_float))
                }
            } else {
                Ok(Value::Int(min_int))
            }
        })),
        "max" => Some(Function::new(|argument| {
            let arguments = argument.as_tuple()?;
            let mut max_int = IntType::min_value();
            let mut max_float = Option::<FloatType>::None;

            for argument in arguments {
                if let Value::Float(float) = argument {
                    if let Some(max) = max_float {
                        max_float = Some(max.max(float));
                    } else {
                        max_float = Some(float);
                    }
                } else if let Value::Int(int) = argument {
                    max_int = max_int.max(int);
                } else {
                    return Err(EvalexprError::expected_number(argument));
                }
            }

            if let Some(max_float) = max_float {
                if ({
                    #[cfg(feature = "decimal_support")]
                    {
                        FloatType::from_i64(max_int).unwrap()
                    }
                    #[cfg(not(feature = "decimal_support"))]
                    {
                        max_int as FloatType
                    }
                }) > max_float
                {
                    Ok(Value::Int(max_int))
                } else {
                    Ok(Value::Float(max_float))
                }
            } else {
                Ok(Value::Int(max_int))
            }
        })),
        "if" => Some(Function::new(|argument| {
            let mut arguments = argument.as_fixed_len_tuple(3)?;
            let result_index = if arguments[0].as_boolean()? { 1 } else { 2 };
            Ok(arguments.swap_remove(result_index))
        })),
        "contains" => Some(Function::new(move |argument| {
            let arguments = argument.as_fixed_len_tuple(2)?;
            if let (Value::Tuple(a), b) = (&arguments[0].clone(), &arguments[1].clone()) {
                if let Value::String(_) | Value::Int(_) | Value::Float(_) | Value::Boolean(_) = b {
                    Ok(a.contains(b).into())
                } else {
                    Err(EvalexprError::type_error(
                        b.clone(),
                        vec![
                            ValueType::String,
                            ValueType::Int,
                            ValueType::Float,
                            ValueType::Boolean,
                        ],
                    ))
                }
            } else {
                Err(EvalexprError::expected_tuple(arguments[0].clone()))
            }
        })),
        "contains_any" => Some(Function::new(move |argument| {
            let arguments = argument.as_fixed_len_tuple(2)?;
            if let (Value::Tuple(a), b) = (&arguments[0].clone(), &arguments[1].clone()) {
                if let Value::Tuple(b) = b {
                    let mut contains = false;
                    for value in b {
                        if let Value::String(_)
                        | Value::Int(_)
                        | Value::Float(_)
                        | Value::Boolean(_) = value
                        {
                            if a.contains(value) {
                                contains = true;
                            }
                        } else {
                            return Err(EvalexprError::type_error(
                                value.clone(),
                                vec![
                                    ValueType::String,
                                    ValueType::Int,
                                    ValueType::Float,
                                    ValueType::Boolean,
                                ],
                            ));
                        }
                    }
                    Ok(contains.into())
                } else {
                    Err(EvalexprError::expected_tuple(b.clone()))
                }
            } else {
                Err(EvalexprError::expected_tuple(arguments[0].clone()))
            }
        })),
        "len" => Some(Function::new(|argument| {
            if let Ok(subject) = argument.as_string() {
                Ok(Value::from(subject.len() as IntType))
            } else if let Ok(subject) = argument.as_tuple() {
                Ok(Value::from(subject.len() as IntType))
            } else {
                Err(EvalexprError::type_error(
                    argument.clone(),
                    vec![ValueType::String, ValueType::Tuple],
                ))
            }
        })),
        // String functions
        #[cfg(feature = "regex_support")]
        "str::regex_matches" => Some(Function::new(|argument| {
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
        })),
        #[cfg(feature = "regex_support")]
        "str::regex_replace" => Some(Function::new(|argument| {
            let arguments = argument.as_tuple()?;

            let subject = arguments[0].as_string()?;
            let re_str = arguments[1].as_string()?;
            let repl = arguments[2].as_string()?;
            match Regex::new(&re_str) {
                Ok(re) => Ok(Value::String(
                    re.replace_all(&subject, repl.as_str()).to_string(),
                )),
                Err(err) => Err(EvalexprError::invalid_regex(
                    re_str.to_string(),
                    format!("{}", err),
                )),
            }
        })),
        "str::to_lowercase" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_lowercase()))
        })),
        "str::to_uppercase" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_uppercase()))
        })),
        "str::trim" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.trim()))
        })),
        "str::from" => Some(Function::new(|argument| {
            Ok(Value::String(argument.to_string()))
        })),
        #[cfg(feature = "rand")]
        "random" => Some(Function::new(|argument| {
            argument.as_empty()?;
            Ok(Value::Float({
                #[cfg(feature = "decimal_support")]
                {
                    FloatType::from_f64(rand::random()).unwrap()
                }
                #[cfg(not(feature = "decimal_support"))]
                {
                    rand::random()
                }
            }))
        })),
        // Bitwise operators
        "bitand" => int_function!(bitand, 2),
        "bitor" => int_function!(bitor, 2),
        "bitxor" => int_function!(bitxor, 2),
        "bitnot" => int_function!(not),
        "shl" => int_function!(shl, 2),
        "shr" => int_function!(shr, 2),
        _ => None,
    }
}
