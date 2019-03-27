use EvalexprError;
use Function;
use value::{FloatType, IntType};
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
        _ => None,
    }
}
