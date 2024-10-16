use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::{EvalexprError, EvalexprResult, Value};

pub trait EvalexprNumericTypes: 'static + Sized + Debug + Clone + PartialEq {
    type Int: EvalexprInt<Self>;
    type Float: EvalexprFloat<Self>;

    /// Convert an integer to a float using the `as` operator or a similar mechanic.
    fn int_as_float(int: &Self::Int) -> Self::Float;

    /// Convert a float to an integer using the `as` operator or a similar mechanic.
    fn float_as_int(float: &Self::Float) -> Self::Int;
}

pub trait EvalexprInt<NumericTypes: EvalexprNumericTypes<Int = Self>>:
    Clone + Debug + Display + FromStr + Eq + Ord
{
    const MIN: Self;
    const MAX: Self;

    /// Convert a value of type [`usize`] into `Self`.
    fn from_usize(int: usize) -> EvalexprResult<Self, NumericTypes>;

    /// Convert `self` into [`usize`].
    fn into_usize(&self) -> EvalexprResult<usize, NumericTypes>;

    /// Parse `Self` from a hex string.
    fn from_hex_str(literal: &str) -> Result<Self, ()>;

    fn checked_add(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    fn checked_sub(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    fn checked_neg(&self) -> EvalexprResult<Self, NumericTypes>;

    fn checked_mul(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    fn checked_div(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    fn checked_rem(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    fn abs(&self) -> EvalexprResult<Self, NumericTypes>;

    fn bitand(&self, rhs: &Self) -> Self;

    fn bitor(&self, rhs: &Self) -> Self;

    fn bitxor(&self, rhs: &Self) -> Self;

    fn bitnot(&self) -> Self;

    fn bit_shift_left(&self, rhs: &Self) -> Self;

    fn bit_shift_right(&self, rhs: &Self) -> Self;
}

pub trait EvalexprFloat<NumericTypes: EvalexprNumericTypes<Float = Self>>:
    Clone
    + Debug
    + Display
    + FromStr
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
    const MIN: Self;
    const MAX: Self;

    fn pow(&self, exponent: &Self) -> Self;

    fn ln(&self) -> Self;

    fn log(&self, base: &Self) -> Self;

    fn log2(&self) -> Self;

    fn log10(&self) -> Self;

    fn exp(&self) -> Self;

    fn exp2(&self) -> Self;

    fn cos(&self) -> Self;

    fn cosh(&self) -> Self;

    fn acos(&self) -> Self;

    fn acosh(&self) -> Self;

    fn sin(&self) -> Self;

    fn sinh(&self) -> Self;

    fn asin(&self) -> Self;

    fn asinh(&self) -> Self;

    fn tan(&self) -> Self;

    fn tanh(&self) -> Self;

    fn atan(&self) -> Self;

    fn atanh(&self) -> Self;

    fn atan2(&self, s: &Self) -> Self;

    fn sqrt(&self) -> Self;

    fn cbrt(&self) -> Self;

    fn hypot(&self, other: &Self) -> Self;

    fn floor(&self) -> Self;

    fn round(&self) -> Self;

    fn ceil(&self) -> Self;

    fn is_nan(&self) -> bool;

    fn is_finite(&self) -> bool;

    fn is_infinite(&self) -> bool;

    fn is_normal(&self) -> bool;

    fn abs(&self) -> Self;

    fn min(&self, other: &Self) -> Self;

    fn max(&self, other: &Self) -> Self;

    /// Generate a random float value between 0.0 and 1.0.
    ///
    /// If the feature `rand` is not enabled, then this method always returns [`EvalexprError::RandNotEnabled`].
    fn random() -> EvalexprResult<Self, NumericTypes>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DefaultNumericTypes;

impl EvalexprNumericTypes for DefaultNumericTypes {
    type Int = i64;
    type Float = f64;

    fn int_as_float(int: &Self::Int) -> Self::Float {
        *int as Self::Float
    }

    fn float_as_int(float: &Self::Float) -> Self::Int {
        *float as Self::Int
    }
}

impl<NumericTypes: EvalexprNumericTypes<Int = Self>> EvalexprInt<NumericTypes> for i64 {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    fn from_usize(int: usize) -> EvalexprResult<Self, NumericTypes> {
        int.try_into()
            .map_err(|_| EvalexprError::IntFromUsize { usize_int: int })
    }

    fn into_usize(&self) -> EvalexprResult<usize, NumericTypes> {
        if *self >= 0 {
            (*self as u64)
                .try_into()
                .map_err(|_| EvalexprError::IntIntoUsize { int: self.clone() })
        } else {
            Err(EvalexprError::IntIntoUsize { int: self.clone() })
        }
    }

    fn from_hex_str(literal: &str) -> Result<Self, ()> {
        Self::from_str_radix(literal, 16).map_err(|_| ())
    }

    fn checked_add(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_add(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::addition_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_sub(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_sub(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::subtraction_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_neg(&self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_neg();
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::negation_error(
                Value::<NumericTypes>::from_int(*self),
            ))
        }
    }

    fn checked_mul(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_mul(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::multiplication_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_div(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_div(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::division_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn checked_rem(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes> {
        let result = (*self).checked_rem(*rhs);
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(EvalexprError::modulation_error(
                Value::<NumericTypes>::from_int(*self),
                Value::<NumericTypes>::from_int(*rhs),
            ))
        }
    }

    fn abs(&self) -> EvalexprResult<Self, NumericTypes> {
        todo!()
    }

    fn bitand(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn bitor(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn bitxor(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn bitnot(&self) -> Self {
        todo!()
    }

    fn bit_shift_left(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn bit_shift_right(&self, rhs: &Self) -> Self {
        todo!()
    }
}

impl<NumericTypes: EvalexprNumericTypes<Float = Self>> EvalexprFloat<NumericTypes> for f64 {
    const MIN: Self = Self::NEG_INFINITY;
    const MAX: Self = Self::INFINITY;

    fn pow(&self, exponent: &Self) -> Self {
        (*self).powf(*exponent)
    }

    fn ln(&self) -> Self {
        (*self).ln()
    }
}
