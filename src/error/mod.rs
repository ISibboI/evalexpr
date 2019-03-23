//! The `error` module contains the `Error` enum that contains all error types used by this crate.
//!
//! The `Error` enum implements constructors for its struct variants, because those are ugly to construct.
//!
//! The module also contains some helper functions starting with `expect_` that check for a condition and return `Err(_)` if the condition is not fulfilled.
//! They are meant as shortcuts to not write the same error checking code everywhere.

use crate::value::Value;
use token::PartialToken;
use value::TupleType;

mod display;

/// Errors used in this crate.
#[derive(Debug, PartialEq)]
pub enum Error {
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
        expected: usize,
        /// The actual amount of arguments.
        actual: usize,
    },

    /// A string value was expected.
    ExpectedString {
        /// The actual value.
        actual: Value,
    },

    /// An integer value was expected.
    ExpectedInt {
        /// The actual value.
        actual: Value,
    },

    /// A float value was expected.
    ExpectedFloat {
        /// The actual value.
        actual: Value,
    },

    /// A numeric value was expected.
    /// Numeric values are the variants `Value::Int` and `Value::Float`.
    ExpectedNumber {
        /// The actual value.
        actual: Value,
    },

    /// A boolean value was expected.
    ExpectedBoolean {
        /// The actual value.
        actual: Value,
    },

    /// A tuple value was expected.
    ExpectedTuple {
        /// The actual value.
        actual: Value,
    },

    /// The given expression is empty
    EmptyExpression,

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode,

    /// Tried to append a child to a node such that the precedence of the child is not higher.
    PrecedenceViolation,

    /// A `VariableIdentifier` operation did not find its value in the configuration.
    VariableIdentifierNotFound(String),

    /// A `FunctionIdentifier` operation did not find its value in the configuration.
    FunctionIdentifierNotFound(String),

    /// A value has the wrong type.
    /// Only use this if there is no other error that describes the expected and provided types in more detail.
    TypeError {
        /// The expected types.
        expected: TupleType,
        /// The actual value.
        actual: Value,
    },

    /// An opening brace without a matching closing brace was found.
    UnmatchedLBrace,

    /// A closing brace without a matching opening brace was found.
    UnmatchedRBrace,

    /// A `PartialToken` is unmatched, such that it cannot be combined into a full `Token`.
    /// This happens if for example a single `=` is found, surrounded by whitespace.
    /// It is not a token, but it is part of the string representation of some tokens.
    UnmatchedPartialToken {
        /// The unmatched partial token.
        first: PartialToken,
        /// The token that follows the unmatched partial token and that cannot be matched to the partial token, or `None`, if `first` is the last partial token in the stream.
        second: Option<PartialToken>,
    },

    /// An addition operation performed by Rust failed.
    AdditionError {
        /// The first argument of the addition.
        augend: Value,
        /// The second argument of the addition.
        addend: Value,
    },

    /// A subtraction operation performed by Rust failed.
    SubtractionError {
        /// The first argument of the subtraction.
        minuend: Value,
        /// The second argument of the subtraction.
        subtrahend: Value,
    },

    /// A negation operation performed by Rust failed.
    NegationError {
        /// The argument of the negation.
        argument: Value,
    },

    /// A multiplication operation performed by Rust failed.
    MultiplicationError {
        /// The first argument of the multiplication.
        multiplicand: Value,
        /// The second argument of the multiplication.
        multiplier: Value,
    },

    /// A division operation performed by Rust failed.
    DivisionError {
        /// The first argument of the division.
        dividend: Value,
        /// The second argument of the division.
        divisor: Value,
    },

    /// A modulation operation performed by Rust failed.
    ModulationError {
        /// The first argument of the modulation.
        dividend: Value,
        /// The second argument of the modulation.
        divisor: Value,
    },

    /// A custom error explained by its message.
    CustomMessage(String),
}

impl Error {
    pub(crate) fn wrong_operator_argument_amount(actual: usize, expected: usize) -> Self {
        Error::WrongOperatorArgumentAmount { actual, expected }
    }

    pub(crate) fn wrong_function_argument_amount(actual: usize, expected: usize) -> Self {
        Error::WrongFunctionArgumentAmount { actual, expected }
    }

    /// Constructs `Error::TypeError{actual, expected}`.
    pub fn type_error(actual: Value, expected: TupleType) -> Self {
        Error::TypeError { actual, expected }
    }

    /// Constructs `Error::ExpectedString(actual)`.
    pub fn expected_string(actual: Value) -> Self {
        Error::ExpectedString { actual }
    }

    /// Constructs `Error::ExpectedInt(actual)`.
    pub fn expected_int(actual: Value) -> Self {
        Error::ExpectedInt { actual }
    }

    /// Constructs `Error::ExpectedFloat(actual)`.
    pub fn expected_float(actual: Value) -> Self {
        Error::ExpectedFloat { actual }
    }

    /// Constructs `Error::ExpectedNumber(actual)`.
    pub fn expected_number(actual: Value) -> Self {
        Error::ExpectedNumber { actual }
    }

    /// Constructs `Error::ExpectedBoolean(actual)`.
    pub fn expected_boolean(actual: Value) -> Self {
        Error::ExpectedBoolean { actual }
    }

    /// Constructs `Error::ExpectedTuple(actual)`.
    pub fn expected_tuple(actual: Value) -> Self {
        Error::ExpectedTuple { actual }
    }

    pub(crate) fn unmatched_partial_token(
        first: PartialToken,
        second: Option<PartialToken>,
    ) -> Self {
        Error::UnmatchedPartialToken { first, second }
    }

    pub(crate) fn addition_error(augend: Value, addend: Value) -> Self {
        Error::AdditionError { augend, addend }
    }

    pub(crate) fn subtraction_error(minuend: Value, subtrahend: Value) -> Self {
        Error::SubtractionError {
            minuend,
            subtrahend,
        }
    }

    pub(crate) fn negation_error(argument: Value) -> Self {
        Error::NegationError { argument }
    }

    pub(crate) fn multiplication_error(multiplicand: Value, multiplier: Value) -> Self {
        Error::MultiplicationError {
            multiplicand,
            multiplier,
        }
    }

    pub(crate) fn division_error(dividend: Value, divisor: Value) -> Self {
        Error::DivisionError { dividend, divisor }
    }

    pub(crate) fn modulation_error(dividend: Value, divisor: Value) -> Self {
        Error::ModulationError { dividend, divisor }
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongOperatorArgumentAmount)` otherwise.
pub(crate) fn expect_operator_argument_amount(actual: usize, expected: usize) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::wrong_operator_argument_amount(actual, expected))
    }
}

/// Returns `Ok(())` if the actual and expected parameters are equal, and `Err(Error::WrongFunctionArgumentAmount)` otherwise.
pub(crate) fn expect_function_argument_amount(actual: usize, expected: usize) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::wrong_function_argument_amount(actual, expected))
    }
}

/// Returns `Ok(())` if the given value is numeric.
/// Numeric types are `Value::Int` and `Value::Float`.
/// Otherwise, `Err(Error::ExpectedNumber)` is returned.
pub fn expect_number(actual: &Value) -> Result<(), Error> {
    match actual {
        Value::Float(_) | Value::Int(_) => Ok(()),
        _ => Err(Error::expected_number(actual.clone())),
    }
}

/// Returns `Ok(())` if the given value is a `Value::Boolean`, or `Err(Error::ExpectedBoolean)` otherwise.
pub fn expect_boolean(actual: &Value) -> Result<bool, Error> {
    match actual {
        Value::Boolean(boolean) => Ok(*boolean),
        _ => Err(Error::expected_boolean(actual.clone())),
    }
}

impl std::error::Error for Error {}
