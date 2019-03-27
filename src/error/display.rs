use std::fmt;

use EvalexprError;

impl fmt::Display for EvalexprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use EvalexprError::*;
        match self {
            WrongOperatorArgumentAmount { expected, actual } => write!(
                f,
                "An operator expected {} arguments, but got {}.",
                expected, actual
            ),
            WrongFunctionArgumentAmount { expected, actual } => write!(
                f,
                "A function expected {} arguments, but got {}.",
                expected, actual
            ),
            ExpectedString { actual } => {
                write!(f, "Expected a Value::String, but got {:?}.", actual)
            },
            ExpectedInt { actual } => write!(f, "Expected a Value::Int, but got {:?}.", actual),
            ExpectedFloat { actual } => write!(f, "Expected a Value::Float, but got {:?}.", actual),
            ExpectedNumber { actual } => {
                write!(f, "Expected a Value::Number, but got {:?}.", actual)
            },
            ExpectedBoolean { actual } => {
                write!(f, "Expected a Value::Boolean, but got {:?}.", actual)
            },
            ExpectedTuple { actual } => write!(f, "Expected a Value::Tuple, but got {:?}.", actual),
            EmptyExpression => write!(
                f,
                "Got an empty expression that cannot be parsed into a node tree, because it \
                 returns nothing."
            ),
            AppendedToLeafNode => write!(f, "Tried to append a node to a leaf node."),
            PrecedenceViolation => write!(
                f,
                "Tried to append a node to another node with higher precedence."
            ),
            VariableIdentifierNotFound(identifier) => write!(
                f,
                "Variable identifier is not bound to anything by configuration: {:?}.",
                identifier
            ),
            FunctionIdentifierNotFound(identifier) => write!(
                f,
                "Function identifier is not bound to anything by configuration: {:?}.",
                identifier
            ),
            TypeError { expected, actual } => {
                write!(f, "Expected one of {:?}, but got {:?}.", expected, actual)
            },
            UnmatchedLBrace => write!(f, "Found an unmatched opening parenthesis '('."),
            UnmatchedRBrace => write!(f, "Found an unmatched closing parenthesis ')'."),
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
            ContextNotManipulable => write!(f, "Cannot manipulate context"),
            CustomMessage(message) => write!(f, "Error: {}", message),
        }
    }
}
