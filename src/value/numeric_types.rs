use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    str::FromStr,
};

use crate::EvalexprResult;

pub mod default_numeric_types;
/*#[cfg(feature = "num-traits")]
pub mod num_traits_numeric_types;*/

/// A trait to parameterise `evalexpr` with an int type and a float type.
///
/// See [`EvalexprInt`] and [`EvalexprFloat`] for the requirements on the types.
pub trait EvalexprNumericTypes: 'static + Sized + Debug + Clone + PartialEq {
    /// The integer type.
    #[cfg(feature = "serde")]
    type Int: EvalexprInt<Self> + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The integer type.
    #[cfg(not(feature = "serde"))]
    type Int: EvalexprInt<Self>;

    /// The float type.
    #[cfg(feature = "serde")]
    type Float: EvalexprFloat<Self> + serde::Serialize + for<'de> serde::Deserialize<'de>;

    /// The float type.
    #[cfg(not(feature = "serde"))]
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
    #[expect(clippy::wrong_self_convention)]
    fn into_usize(&self) -> EvalexprResult<usize, NumericTypes>;

    /// Parse `Self` from a hex string.
    #[expect(clippy::result_unit_err)]
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
    /// If the feature `rand` is not enabled, then this method always returns [`EvalexprError::RandNotEnabled`](crate::EvalexprError::RandNotEnabled).
    fn random() -> EvalexprResult<Self, NumericTypes>;
}
