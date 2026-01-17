use std::fmt;

use crate::{value::numeric_types::EvalexprNumericTypes, EvalexprError};

impl<NumericTypes: EvalexprNumericTypes> fmt::Display for EvalexprError<NumericTypes> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use crate::EvalexprError::*;
        match self {
            WrongOperatorArgumentAmount { expected, actual } => write!(
                f,
                "An operator expected {} arguments, but got {}.",
                expected, actual
            ),
            WrongFunctionArgumentAmount { expected, actual } => {
                let expected_arguments = if expected.start() == expected.end() {
                    format!("{}", expected.start())
                } else {
                    format!("{} to {}", expected.start(), expected.end())
                };
                write!(
                    f,
                    "A function expected {} arguments, but got {}.",
                    expected_arguments, actual
                )
            },
            ExpectedString { actual } => {
                write!(f, "Expected a Value::String, but got {:?}.", actual)
            },
            ExpectedInt { actual } => write!(f, "Expected a Value::Int, but got {:?}.", actual),
            ExpectedFloat { actual } => write!(f, "Expected a Value::Float, but got {:?}.", actual),
            ExpectedNumber { actual } => write!(
                f,
                "Expected a Value::Float or Value::Int, but got {:?}.",
                actual
            ),
            ExpectedNumberOrString { actual } => write!(
                f,
                "Expected a Value::Number or a Value::String, but got {:?}.",
                actual
            ),
            ExpectedBoolean { actual } => {
                write!(f, "Expected a Value::Boolean, but got {:?}.", actual)
            },
            ExpectedTuple { actual } => write!(f, "Expected a Value::Tuple, but got {:?}.", actual),
            ExpectedFixedLengthTuple {
                expected_length,
                actual,
            } => write!(
                f,
                "Expected a Value::Tuple of length {}, but got {:?}.",
                expected_length, actual
            ),
            ExpectedRangedLengthTuple {
                expected_length,
                actual,
            } => write!(
                f,
                "Expected a Value::Tuple of length {} to {}, but got {:?}.",
                expected_length.start(),
                expected_length.end(),
                actual
            ),
            ExpectedEmpty { actual } => write!(f, "Expected a Value::Empty, but got {:?}.", actual),
            AppendedToLeafNode => write!(f, "Tried to append a node to a leaf node."),
            PrecedenceViolation => write!(
                f,
                "Tried to append a node to another node with higher precedence."
            ),
            VariableIdentifierNotFound(identifier) => write!(
                f,
                "Variable identifier is not bound to anything by context: {:?}.",
                identifier
            ),
            FunctionIdentifierNotFound(identifier) => write!(
                f,
                "Function identifier is not bound to anything by context: {:?}.",
                identifier
            ),
            TypeError { expected, actual } => {
                write!(f, "Expected one of {:?}, but got {:?}.", expected, actual)
            },
            WrongTypeCombination { operator, actual } => write!(
                f,
                "The operator {:?} was called with a wrong combination of types: {:?}",
                operator, actual
            ),
            UnmatchedLBrace => write!(f, "Found an unmatched opening parenthesis '('."),
            UnmatchedRBrace => write!(f, "Found an unmatched closing parenthesis ')'."),
            UnmatchedDoubleQuote => write!(f, "Found an unmatched double quote '\"'"),
            MissingOperatorOutsideOfBrace => write!(
                f,
                "Found an opening parenthesis that is preceded by something that does not take \
                 any arguments on the right, or found a closing parenthesis that is succeeded by \
                 something that does not take any arguments on the left."
            ),
            UnmatchedPartialToken { first, second } => {
                if let Some(second) = second {
                    write!(
                        f,
                        "Found a partial token '{}' that should not be followed by '{}'.",
                        first, second
                    )
                } else {
                    write!(
                        f,
                        "Found a partial token '{}' that should be followed by another partial \
                         token.",
                        first
                    )
                }
            },
            AdditionError { augend, addend } => write!(f, "Error adding {} + {}", augend, addend),
            SubtractionError {
                minuend,
                subtrahend,
            } => write!(f, "Error subtracting {} - {}", minuend, subtrahend),
            NegationError { argument } => write!(f, "Error negating -{}", argument),
            MultiplicationError {
                multiplicand,
                multiplier,
            } => write!(f, "Error multiplying {} * {}", multiplicand, multiplier),
            DivisionError { dividend, divisor } => {
                write!(f, "Error dividing {} / {}", dividend, divisor)
            },
            ModulationError { dividend, divisor } => {
                write!(f, "Error modulating {} % {}", dividend, divisor)
            },
            InvalidRegex { regex, message } => write!(
                f,
                "Regular expression {:?} is invalid: {:?}",
                regex, message
            ),
            ContextNotMutable => write!(f, "Cannot manipulate context"),
            BuiltinFunctionsCannotBeEnabled => {
                write!(f, "This context does not allow enabling builtin functions")
            },
            BuiltinFunctionsCannotBeDisabled => {
                write!(f, "This context does not allow disabling builtin functions")
            },
            IllegalEscapeSequence(string) => write!(f, "Illegal escape sequence: {}", string),
            OutOfBoundsAccess => write!(f, "Tried to access a tuple or string at an invalid index"),
            IntFromUsize { usize_int } => write!(
                f,
                "The usize {} does not fit into the chosen integer type",
                usize_int
            ),
            IntIntoUsize { int } => write!(
                f,
                "The int {} does not fit into an usize on this platform",
                int
            ),
            RandNotEnabled => write!(f, "The feature 'rand' must be enabled to use randomness"),
            UnsuitableOperator(operator) => write!(f, "Unsuitable operator used: {operator}"),
            CustomMessage(message) => write!(f, "Error: {}", message),
        }
    }
}
