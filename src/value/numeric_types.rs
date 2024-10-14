use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    ops::Mul,
    str::FromStr,
};

use crate::{EvalexprError, EvalexprResult, Value};

pub trait EvalexprNumericTypes: Sized + Debug + Clone + PartialEq {
    type Int: Clone + Debug + Display + FromStr + Eq;
    type Float: Clone + Debug + Display + FromStr + PartialEq + Mul;

    const MIN_INT: Self::Int;
    const MIN_FLOAT: Self::Float;
    const MAX_INT: Self::Int;
    const MAX_FLOAT: Self::Float;

    /// Convert an integer to a float using the `as` operator or a similar mechanic.
    fn int_as_float(int: &Self::Int) -> Self::Float;

    /// Convert a float to an integer using the `as` operator or a similar mechanic.
    fn float_as_int(float: &Self::Float) -> Self::Int;

    /// Convert a value of type [`usize`] into a [`Self::Int`].
    fn int_from_usize(int: usize) -> EvalexprResult<Self::Int, Self>;

    /// Parse an integer from a hex string.
    fn int_from_hex_str(literal: &str) -> Result<Self::Int, ()>;

    fn int_checked_mul(a: &Self::Int, b: &Self::Int) -> EvalexprResult<Self::Int, Self>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DefaultNumericTypes;

impl EvalexprNumericTypes for DefaultNumericTypes {
    type Int = i64;
    type Float = f64;

    const MIN_INT: Self::Int = Self::Int::MIN;
    const MIN_FLOAT: Self::Float = Self::Float::NEG_INFINITY;
    const MAX_INT: Self::Int = Self::Int::MAX;
    const MAX_FLOAT: Self::Float = Self::Float::INFINITY;

    fn int_as_float(int: &Self::Int) -> Self::Float {
        *int as Self::Float
    }

    fn float_as_int(float: &Self::Float) -> Self::Int {
        *float as Self::Int
    }

    fn int_from_usize(int: usize) -> EvalexprResult<Self::Int, Self> {
        int.try_into()
            .map_err(|_| EvalexprError::IntFromUsize { usize_int: int })
    }

    fn int_from_hex_str(literal: &str) -> Result<Self::Int, ()> {
        Self::Int::from_str_radix(literal, 16).map_err(|_| ())
    }

    fn int_checked_mul(a: &Self::Int, b: &Self::Int) -> EvalexprResult<Self::Int, Self> {
        let result = a.checked_mul(*b);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::multiplication_error(
                Value::<Self>::from_int(*a),
                Value::<Self>::from_int(*b),
            ))
        }
    }
}
