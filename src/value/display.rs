use std::fmt::{Display, Error, Formatter};

use crate::Value;

use super::numeric_types::EvalexprNumericTypes;

impl<NumericTypes: EvalexprNumericTypes> Display for Value<NumericTypes> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Float(float) => {
                if let Some(precision) = f.precision() {
                    write!(f, "{1:.*}", precision, float)
                } else {
                    write!(f, "{}", float)
                }
            },
            Value::Int(int) => write!(f, "{}", int),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::Tuple(tuple) => {
                write!(f, "(")?;
                let mut once = false;
                for value in tuple {
                    if once {
                        write!(f, ", ")?;
                    } else {
                        once = true;
                    }
                    value.fmt(f)?;
                }
                write!(f, ")")
            },
            Value::Empty => write!(f, "()"),
        }
    }
}
