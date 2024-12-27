use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

use crate::{EvalexprError, EvalexprResult, Value};

use super::{EvalexprFloat, EvalexprInt, EvalexprNumericTypes};

/// See [`EvalexprNumericTypes`].
///
/// This empty struct uses [`i64`] as its integer type and [`f64`] as its float type.
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
                .map_err(|_| EvalexprError::IntIntoUsize { int: *self })
        } else {
            Err(EvalexprError::IntIntoUsize { int: *self })
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
        Ok((*self).abs())
    }

    fn bitand(&self, rhs: &Self) -> Self {
        BitAnd::bitand(*self, *rhs)
    }

    fn bitor(&self, rhs: &Self) -> Self {
        BitOr::bitor(*self, *rhs)
    }

    fn bitxor(&self, rhs: &Self) -> Self {
        BitXor::bitxor(*self, *rhs)
    }

    fn bitnot(&self) -> Self {
        Not::not(*self)
    }

    fn bit_shift_left(&self, rhs: &Self) -> Self {
        Shl::shl(*self, *rhs)
    }

    fn bit_shift_right(&self, rhs: &Self) -> Self {
        Shr::shr(*self, *rhs)
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

    fn log(&self, base: &Self) -> Self {
        (*self).log(*base)
    }

    fn log2(&self) -> Self {
        (*self).log2()
    }

    fn log10(&self) -> Self {
        (*self).log10()
    }

    fn exp(&self) -> Self {
        (*self).exp()
    }

    fn exp2(&self) -> Self {
        (*self).exp2()
    }

    fn cos(&self) -> Self {
        (*self).cos()
    }

    fn cosh(&self) -> Self {
        (*self).cosh()
    }

    fn acos(&self) -> Self {
        (*self).acos()
    }

    fn acosh(&self) -> Self {
        (*self).acosh()
    }

    fn sin(&self) -> Self {
        (*self).sin()
    }

    fn sinh(&self) -> Self {
        (*self).sinh()
    }

    fn asin(&self) -> Self {
        (*self).asin()
    }

    fn asinh(&self) -> Self {
        (*self).asinh()
    }

    fn tan(&self) -> Self {
        (*self).tan()
    }

    fn tanh(&self) -> Self {
        (*self).tanh()
    }

    fn atan(&self) -> Self {
        (*self).atan()
    }

    fn atanh(&self) -> Self {
        (*self).atanh()
    }

    fn atan2(&self, x: &Self) -> Self {
        (*self).atan2(*x)
    }

    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }

    fn cbrt(&self) -> Self {
        (*self).cbrt()
    }

    fn hypot(&self, other: &Self) -> Self {
        (*self).hypot(*other)
    }

    fn floor(&self) -> Self {
        (*self).floor()
    }

    fn round(&self) -> Self {
        (*self).round()
    }

    fn ceil(&self) -> Self {
        (*self).ceil()
    }

    fn is_nan(&self) -> bool {
        (*self).is_nan()
    }

    fn is_finite(&self) -> bool {
        (*self).is_finite()
    }

    fn is_infinite(&self) -> bool {
        (*self).is_infinite()
    }

    fn is_normal(&self) -> bool {
        (*self).is_normal()
    }

    fn abs(&self) -> Self {
        (*self).abs()
    }

    fn min(&self, other: &Self) -> Self {
        (*self).min(*other)
    }

    fn max(&self, other: &Self) -> Self {
        (*self).max(*other)
    }

    fn random() -> EvalexprResult<Self, NumericTypes> {
        #[cfg(feature = "rand")]
        let result = Ok(rand::random());

        #[cfg(not(feature = "rand"))]
        let result = Err(EvalexprError::RandNotEnabled);

        result
    }
}
