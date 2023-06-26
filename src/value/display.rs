use std::fmt::{Display, Error, Formatter};

use crate::Value;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Float(float) => {
                #[cfg(feature = "decimal_support")]
                {
                    if float.is_integer() || float.is_zero() {
                        write!(f, "{:.0}", float)
                    } else {
                        write!(f, "{}", float)
                    }
                }
                #[cfg(not(feature = "decimal_support"))]
                {
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
