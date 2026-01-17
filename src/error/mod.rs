//! The `error` module contains the `Error` enum that contains all error types used by this crate.
//!
//! The `Error` enum implements constructors for its struct variants, because those are ugly to construct.
//!
//! The module also contains some helper functions starting with `expect_` that check for a condition and return `Err(_)` if the condition is not fulfilled.
//! They are meant as shortcuts to not write the same error checking code everywhere.

use std::ops::RangeInclusive;

use crate::{
    token::PartialToken,
    value::{
        numeric_types::{default_numeric_types::DefaultNumericTypes, EvalexprNumericTypes},
        value_type::ValueType,
    },
};

use crate::{operator::Operator, value::Value};

// Exclude error display code from test coverage, as the code does not make sense to test.
#[cfg(not(tarpaulin_include))]
mod display;

/// Errors used in this crate.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum EvalexprError<NumericTypes: EvalexprNumericTypes = DefaultNumericTypes> {
    /// An operator was called with a wrong amount of arguments.
    WrongOperatorArgumentAmount {
        /// The expected amount of arguments.
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A function was called with a wrong amount of arguments.
    WrongFunctionArgumentAmount {
        /// The expected amount of arguments.
        expected: RangeInclusive<usize>,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A string value was expected.
    ExpectedString {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// An integer value was expected.
    ExpectedInt {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A float value was expected.
    ExpectedFloat {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A numeric value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumber {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A numeric or string value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumberOrString {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A boolean value was expected.
    ExpectedBoolean {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A tuple value was expected.
    ExpectedTuple {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A tuple value of a certain length was expected.
    ExpectedFixedLengthTuple {
        /// The expected length.
        expected_length: usize,
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// A tuple value of a certain length range was expected.
    ExpectedRangedLengthTuple {
        /// The expected length range.
        expected_length: RangeInclusive<usize>,
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// An empty value was expected.
    ExpectedEmpty {
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode,

    /// Tried to append a child to a node such that the precedence of the child is not higher.
    /// This error should never occur.
    /// If it does, please file a bug report.
    PrecedenceViolation,

    /// A `VariableIdentifier` operation did not find its value in the context.
    VariableIdentifierNotFound(String),

    /// A `FunctionIdentifier` operation did not find its value in the context.
    FunctionIdentifierNotFound(String),

    /// A value has the wrong type.
    /// Only use this if there is no other error that describes the expected and provided types in more detail.
    TypeError {
        /// The expected types.
        expected: Vec<ValueType>,
        /// The actual value.
        actual: Value<NumericTypes>,
    },

    /// An operator is used with a wrong combination of types.
    WrongTypeCombination {
        /// The operator that whose evaluation caused the error.
        operator: Operator<NumericTypes>,
        /// The types that were used in the operator causing it to fail.
        actual: Vec<ValueType>,
    },

    /// An opening brace without a matching closing brace was found.
    UnmatchedLBrace,

    /// A closing brace without a matching opening brace was found.
    UnmatchedRBrace,

    /// A double quote without a matching second double quote was found.
    UnmatchedDoubleQuote,

    /// Left of an opening brace or right of a closing brace is a token that does not expect the brace next to it.
    /// For example, writing `4(5)` would yield this error, as the `4` does not have any operands.
    MissingOperatorOutsideOfBrace,

    /// A `PartialToken` is unmatched, such that it cannot be combined into a full `Token`.
    /// This happens if for example a single `=` is found, surrounded by whitespace.
    /// It is not a token, but it is part of the string representation of some tokens.
    UnmatchedPartialToken {
        /// The unmatched partial token.
        first: PartialToken<NumericTypes>,
        /// The token that follows the unmatched partial token and that cannot be matched to the partial token, or `None`, if `first` is the last partial token in the stream.
        second: Option<PartialToken<NumericTypes>>,
    },

    /// An addition operation performed by Rust failed.
    AdditionError {
        /// The first argument of the addition.
        augend: Value<NumericTypes>,
        /// The second argument of the addition.
        addend: Value<NumericTypes>,
    },

    /// A subtraction operation performed by Rust failed.
    SubtractionError {
        /// The first argument of the subtraction.
        minuend: Value<NumericTypes>,
        /// The second argument of the subtraction.
        subtrahend: Value<NumericTypes>,
    },

    /// A negation operation performed by Rust failed.
    NegationError {
        /// The argument of the negation.
        argument: Value<NumericTypes>,
    },

    /// A multiplication operation performed by Rust failed.
    MultiplicationError {
        /// The first argument of the multiplication.
        multiplicand: Value<NumericTypes>,
        /// The second argument of the multiplication.
        multiplier: Value<NumericTypes>,
    },

    /// A division operation performed by Rust failed.
    DivisionError {
        /// The first argument of the division.
        dividend: Value<NumericTypes>,
        /// The second argument of the division.
        divisor: Value<NumericTypes>,
    },

    /// A modulation operation performed by Rust failed.
    ModulationError {
        /// The first argument of the modulation.
        dividend: Value<NumericTypes>,
        /// The second argument of the modulation.
        divisor: Value<NumericTypes>,
    },

    /// A regular expression could not be parsed
    InvalidRegex {
        /// The invalid regular expression
        regex: String,
        /// Failure message from the regex engine
        message: String,
    },

    /// A modification was attempted on a `Context` that does not allow modifications.
    ContextNotMutable,

    /// An escape sequence within a string literal is illegal.
    IllegalEscapeSequence(String),

    /// This context does not allow enabling builtin functions.
    BuiltinFunctionsCannotBeEnabled,

    /// This context does not allow disabling builtin functions.
    BuiltinFunctionsCannotBeDisabled,

    /// Out of bounds sequence access.
    OutOfBoundsAccess,

    /// A `usize` was attempted to be converted to an `int`, but it was out of range.
    IntFromUsize {
        /// The `usize` that was attempted to be converted.
        usize_int: usize,
    },

    /// An `int` was attempted to be converted to a `usize`, but it was out of range.
    IntIntoUsize {
        /// The `int` that was attempted to be converted.
        int: NumericTypes::Int,
    },

    /// The feature `rand` is not enabled, but required for the used function.
    RandNotEnabled,

    /// An unsuitable Operator was used
    UnsuitableOperator(Operator<NumericTypes>),

    /// A custom error explained by its message.
    CustomMessage(String),
}

impl<NumericTypes: EvalexprNumericTypes> EvalexprError<NumericTypes> {
    /// Construct a `WrongOperatorArgumentAmount` error.
    pub fn wrong_operator_argument_amount(actual: usize, expected: usize) -> Self {
        EvalexprError::WrongOperatorArgumentAmount { actual, expected }
    }

    /// Construct a `WrongFunctionArgumentAmount` error for a function with a fixed amount of arguments.
    pub fn wrong_function_argument_amount(actual: usize, expected: usize) -> Self {
        EvalexprError::WrongFunctionArgumentAmount {
            actual,
            expected: expected..=expected,
        }
    }

    /// Construct a `WrongFunctionArgumentAmount` error for a function with a range of possible amounts of arguments.
    pub fn wrong_function_argument_amount_range(
        actual: usize,
        expected: RangeInclusive<usize>,
    ) -> Self {
        EvalexprError::WrongFunctionArgumentAmount { actual, expected }
    }

    /// Constructs `EvalexprError::TypeError{actual, expected}`.
    pub fn type_error(actual: Value<NumericTypes>, expected: Vec<ValueType>) -> Self {
        EvalexprError::TypeError { actual, expected }
    }

    /// Constructs `EvalexprError::WrongTypeCombination{operator, actual}`.
    pub fn wrong_type_combination(
        operator: Operator<NumericTypes>,
        actual: Vec<ValueType>,
    ) -> Self {
        EvalexprError::WrongTypeCombination { operator, actual }
    }

    /// Constructs `EvalexprError::ExpectedString{actual}`.
    pub fn expected_string(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedString { actual }
    }

    /// Constructs `EvalexprError::ExpectedInt{actual}`.
    pub fn expected_int(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedInt { actual }
    }

    /// Constructs `EvalexprError::ExpectedFloat{actual}`.
    pub fn expected_float(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedFloat { actual }
    }

    /// Constructs `EvalexprError::ExpectedNumber{actual}`.
    pub fn expected_number(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedNumber { actual }
    }

    /// Constructs `EvalexprError::ExpectedNumberOrString{actual}`.
    pub fn expected_number_or_string(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedNumberOrString { actual }
    }

    /// Constructs `EvalexprError::ExpectedBoolean{actual}`.
    pub fn expected_boolean(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedBoolean { actual }
    }

    /// Constructs `EvalexprError::ExpectedTuple{actual}`.
    pub fn expected_tuple(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedTuple { actual }
    }

    /// Constructs `EvalexprError::ExpectedFixedLenTuple{expected_len, actual}`.
    pub fn expected_fixed_len_tuple(expected_len: usize, actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedFixedLengthTuple {
            expected_length: expected_len,
            actual,
        }
    }

    /// Constructs `EvalexprError::ExpectedFixedLenTuple{expected_len, actual}`.
    pub fn expected_ranged_len_tuple(
        expected_len: RangeInclusive<usize>,
        actual: Value<NumericTypes>,
    ) -> Self {
        EvalexprError::ExpectedRangedLengthTuple {
            expected_length: expected_len,
            actual,
        }
    }

    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_empty(actual: Value<NumericTypes>) -> Self {
        EvalexprError::ExpectedEmpty { actual }
    }

    /// Constructs an error that expresses that the type of `expected` was expected, but `actual` was found.
    pub(crate) fn expected_type(
        expected: &Value<NumericTypes>,
        actual: Value<NumericTypes>,
    ) -> Self {
        match ValueType::from(expected) {
            ValueType::String => Self::expected_string(actual),
            ValueType::Int => Self::expected_int(actual),
            ValueType::Float => Self::expected_float(actual),
            ValueType::Boolean => Self::expected_boolean(actual),
            ValueType::Tuple => Self::expected_tuple(actual),
            ValueType::Empty => Self::expected_empty(actual),
        }
    }

    pub(crate) fn unmatched_partial_token(
        first: PartialToken<NumericTypes>,
        second: Option<PartialToken<NumericTypes>>,
    ) -> Self {
        EvalexprError::UnmatchedPartialToken { first, second }
    }

    pub(crate) fn addition_error(augend: Value<NumericTypes>, addend: Value<NumericTypes>) -> Self {
        EvalexprError::AdditionError { augend, addend }
    }

    pub(crate) fn subtraction_error(
        minuend: Value<NumericTypes>,
        subtrahend: Value<NumericTypes>,
    ) -> Self {
        EvalexprError::SubtractionError {
            minuend,
            subtrahend,
        }
    }

    pub(crate) fn negation_error(argument: Value<NumericTypes>) -> Self {
        EvalexprError::NegationError { argument }
    }

    pub(crate) fn multiplication_error(
        multiplicand: Value<NumericTypes>,
        multiplier: Value<NumericTypes>,
    ) -> Self {
        EvalexprError::MultiplicationError {
            multiplicand,
            multiplier,
        }
    }

    pub(crate) fn division_error(
        dividend: Value<NumericTypes>,
        divisor: Value<NumericTypes>,
    ) -> Self {
        EvalexprError::DivisionError { dividend, divisor }
    }

    pub(crate) fn modulation_error(
        dividend: Value<NumericTypes>,
        divisor: Value<NumericTypes>,
    ) -> Self {
        EvalexprError::ModulationError { dividend, divisor }
    }

    /// Constructs `EvalexprError::InvalidRegex(regex)`
    pub fn invalid_regex(regex: String, message: String) -> Self {
        EvalexprError::InvalidRegex { regex, message }
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongOperatorArgumentAmount)` otherwise.
pub(crate) fn expect_operator_argument_amount<NumericTypes: EvalexprNumericTypes>(
    actual: usize,
    expected: usize,
) -> EvalexprResult<(), NumericTypes> {
    if actual == expected {
        Ok(())
    } else {
        Err(EvalexprError::wrong_operator_argument_amount(
            actual, expected,
        ))
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongFunctionArgumentAmount)` otherwise.
pub fn expect_function_argument_amount<NumericTypes: EvalexprNumericTypes>(
    actual: usize,
    expected: usize,
) -> EvalexprResult<(), NumericTypes> {
    if actual == expected {
        Ok(())
    } else {
        Err(EvalexprError::wrong_function_argument_amount(
            actual, expected,
        ))
    }
}

/// Returns `Ok(())` if the given value is a string or a numeric
pub fn expect_number_or_string<NumericTypes: EvalexprNumericTypes>(
    actual: &Value<NumericTypes>,
) -> EvalexprResult<(), NumericTypes> {
    match actual {
        Value::String(_) | Value::Float(_) | Value::Int(_) => Ok(()),
        _ => Err(EvalexprError::expected_number_or_string(actual.clone())),
    }
}

impl<NumericTypes: EvalexprNumericTypes> std::error::Error for EvalexprError<NumericTypes> {}

/// Standard result type used by this crate.
pub type EvalexprResult<T, NumericTypes = DefaultNumericTypes> =
    Result<T, EvalexprError<NumericTypes>>;

/// Standard result type for [`Value`]s used by this crate.
pub type EvalexprResultValue<NumericTypes = DefaultNumericTypes> =
    EvalexprResult<Value<NumericTypes>, NumericTypes>;

#[cfg(test)]
mod tests {
    use crate::{
        value::numeric_types::default_numeric_types::DefaultNumericTypes, EvalexprError, Value,
        ValueType,
    };

    /// Tests whose only use is to bring test coverage of trivial lines up, like trivial constructors.
    #[test]
    fn trivial_coverage_tests() {
        assert_eq!(
            EvalexprError::type_error(
                Value::<DefaultNumericTypes>::Int(3),
                vec![ValueType::String]
            ),
            EvalexprError::TypeError {
                actual: Value::Int(3),
                expected: vec![ValueType::String]
            }
        );
        assert_eq!(
            EvalexprError::expected_type(
                &Value::<DefaultNumericTypes>::String("abc".to_string()),
                Value::Empty
            ),
            EvalexprError::expected_string(Value::Empty)
        );
        assert_eq!(
            EvalexprError::expected_type(
                &Value::<DefaultNumericTypes>::Boolean(false),
                Value::Empty
            ),
            EvalexprError::expected_boolean(Value::Empty)
        );
        assert_eq!(
            EvalexprError::expected_type(
                &Value::<DefaultNumericTypes>::Tuple(vec![]),
                Value::Empty
            ),
            EvalexprError::expected_tuple(Value::Empty)
        );
        assert_eq!(
            EvalexprError::expected_type(
                &Value::<DefaultNumericTypes>::Empty,
                Value::String("abc".to_string())
            ),
            EvalexprError::expected_empty(Value::String("abc".to_string()))
        );
    }
}
