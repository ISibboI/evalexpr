use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Error {
    WrongArgumentAmount {
        expected: usize,
        actual: usize,
    },
    ExpectedNumber {
        actual: Value,
    },

    /// The given expression is empty
    EmptyExpression,

    /// Tried to evaluate the root node.
    /// The root node should only be used as dummy node.
    EvaluatedRootNode,

    /// Tried to append a child to a leaf node.
    /// Leaf nodes cannot have children.
    AppendedToLeafNode,

    /// Tried to append a child to a node such that the precedence of the child is not higher.
    PrecedenceViolation,

    /// An identifier operation did not find its value in the configuration.
    IdentifierNotFound,

    /// A value has the wrong type.
    TypeError,

    /// An opening brace without a matching closing brace was found.
    UnmatchedLBrace,

    /// A closing brace without a matching opening brace was found.
    UnmatchedRBrace,
}

impl Error {
    pub fn wrong_argument_amount(actual: usize, expected: usize) -> Self {
        Error::WrongArgumentAmount { actual, expected }
    }

    pub fn expected_number(actual: Value) -> Self {
        Error::ExpectedNumber { actual }
    }
}

pub fn expect_argument_amount(actual: usize, expected: usize) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::wrong_argument_amount(actual, expected))
    }
}

pub fn expect_number(actual: &Value) -> Result<(), Error> {
    match actual {
        Value::Float(_) | Value::Int(_) => Ok(()),
        _ => Err(Error::expected_number(actual.clone())),
    }
}
