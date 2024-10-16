use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
    str::FromStr,
};

use crate::{EvalexprError, EvalexprResult, Value};

/// A trait to parameterise `evalexpr` with an int type and a float type.
///
/// See [`EvalexprInt`] and [`EvalexprFloat`] for the requirements on the types.
pub trait EvalexprNumericTypes: 'static + Sized + Debug + Clone + PartialEq {
    /// The integer type.
    type Int: EvalexprInt<Self>;

    /// The float type.
    type Float: EvalexprFloat<Self>;

    /// Convert an integer to a float using the `as` operator or a similar mechanic.
    fn int_as_float(int: &Self::Int) -> Self::Float;

    /// Convert a float to an integer using the `as` operator or a similar mechanic.
    fn float_as_int(float: &Self::Float) -> Self::Int;
}

/// An integer type that can be used by `evalexpr`.
pub trait EvalexprInt<NumericTypes: EvalexprNumericTypes<Int = Self>>:
    Clone + Debug + Display + FromStr + Eq + Ord
{
    /// The minimum value of the integer type.
    const MIN: Self;

    /// The maximum value of the integer type.
    const MAX: Self;

    /// Convert a value of type [`usize`] into `Self`.
    fn from_usize(int: usize) -> EvalexprResult<Self, NumericTypes>;

    /// Convert `self` into [`usize`].
    fn into_usize(&self) -> EvalexprResult<usize, NumericTypes>;

    /// Parse `Self` from a hex string.
    fn from_hex_str(literal: &str) -> Result<Self, ()>;

    /// Perform an addition operation, returning an error on overflow.
    fn checked_add(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a subtraction operation, returning an error on overflow.
    fn checked_sub(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a negation operation, returning an error on overflow.
    fn checked_neg(&self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a multiplication operation, returning an error on overflow.
    fn checked_mul(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a division operation, returning an error on overflow.
    fn checked_div(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a remainder operation, returning an error on overflow.
    fn checked_rem(&self, rhs: &Self) -> EvalexprResult<Self, NumericTypes>;

    /// Compute the absolute value, returning an error on overflow.
    fn abs(&self) -> EvalexprResult<Self, NumericTypes>;

    /// Perform a bitand operation.
    fn bitand(&self, rhs: &Self) -> Self;

    /// Perform a bitor operation.
    fn bitor(&self, rhs: &Self) -> Self;

    /// Perform a bitxor operation.
    fn bitxor(&self, rhs: &Self) -> Self;

    /// Perform a bitnot operation.
    fn bitnot(&self) -> Self;

    /// Perform a shl operation.
    fn bit_shift_left(&self, rhs: &Self) -> Self;

    /// Perform a shr operation.
    fn bit_shift_right(&self, rhs: &Self) -> Self;
}

/// A float type that can be used by `evalexpr`.
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
    /// The smallest non-NaN floating point value.
    ///
    /// Typically, this is negative infinity.
    const MIN: Self;

    /// The largest non-NaN floating point value.
    ///
    /// Typically, this is positive infinity.
    const MAX: Self;

    /// Perform a power operation.
    fn pow(&self, exponent: &Self) -> Self;

    /// Compute the natural logarithm.
    fn ln(&self) -> Self;

    /// Compute the logarithm to a certain base.
    fn log(&self, base: &Self) -> Self;

    /// Compute the logarithm base 2.
    fn log2(&self) -> Self;

    /// Compute the logarithm base 10.
    fn log10(&self) -> Self;

    /// Exponentiate with base `e`.
    fn exp(&self) -> Self;

    /// Exponentiate with base 2.
    fn exp2(&self) -> Self;

    /// Compute the cosine.
    fn cos(&self) -> Self;

    /// Compute the hyperbolic cosine.
    fn cosh(&self) -> Self;

    /// Compute the arccosine.
    fn acos(&self) -> Self;

    /// Compute the hyperbolic arccosine.
    fn acosh(&self) -> Self;

    /// Compute the sine.
    fn sin(&self) -> Self;

    /// Compute the hyperbolic sine.
    fn sinh(&self) -> Self;

    /// Compute the arcsine.
    fn asin(&self) -> Self;

    /// Compute the hyperbolic arcsine.
    fn asinh(&self) -> Self;

    /// Compute the tangent.
    fn tan(&self) -> Self;

    /// Compute the hyperbolic tangent.
    fn tanh(&self) -> Self;

    /// Compute the arctangent.
    fn atan(&self) -> Self;

    /// Compute the hyperbolic arctangent.
    fn atanh(&self) -> Self;

    /// Compute the four quadrant arctangent.
    fn atan2(&self, x: &Self) -> Self;

    /// Compute the square root.
    fn sqrt(&self) -> Self;

    /// Compute the cubic root.
    fn cbrt(&self) -> Self;

    /// Compute the distance between the origin and a point (`self`, `other`) on the Euclidean plane.
    fn hypot(&self, other: &Self) -> Self;

    /// Compute the largest integer less than or equal to `self`.
    fn floor(&self) -> Self;

    /// Returns the nearest integer to `self`. If a value is half-way between two integers, round away from `0.0`.
    fn round(&self) -> Self;

    /// Compute the largest integer greater than or equal to `self`.
    fn ceil(&self) -> Self;

    /// Returns true if `self` is not a number.
    fn is_nan(&self) -> bool;

    /// Returns true if `self` is finite.
    fn is_finite(&self) -> bool;

    /// Returns true if `self` is infinite.
    fn is_infinite(&self) -> bool;

    /// Returns true if `self` is normal.
    fn is_normal(&self) -> bool;

    /// Returns the absolute value of self.
    fn abs(&self) -> Self;

    /// Returns the minimum of the two numbers, ignoring NaN.
    fn min(&self, other: &Self) -> Self;

    /// Returns the maximum of the two numbers, ignoring NaN.
    fn max(&self, other: &Self) -> Self;

    /// Generate a random float value between 0.0 and 1.0.
    ///
    /// If the feature `rand` is not enabled, then this method always returns [`EvalexprError::RandNotEnabled`].
    fn random() -> EvalexprResult<Self, NumericTypes>;
}

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
