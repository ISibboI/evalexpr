use crate::{configuration::Configuration, error::*, value::Value};

pub trait Operator {
    /// Returns the precedence of the operator.
    /// A high precedence means that the operator has priority to be deeper in the tree.
    // Make this a const fn once #57563 is resolved
    fn precedence(&self) -> i32;

    /// True if this operator is a leaf, meaning it accepts no arguments.
    // Make this a const fn once #57563 is resolved
    fn is_leaf(&self) -> bool {
        self.argument_amount() == 0
    }

    /// Returns the amount of arguments required by this operator.
    // Make this a const fn once #57563 is resolved
    fn argument_amount(&self) -> usize;

    /// Evaluates the operator with the given arguments and configuration.
    fn eval(&self, arguments: &[Value], configuration: &Configuration) -> Result<Value, Error>;
}

pub struct RootNode;
pub struct Add;
pub struct Sub;
pub struct Mul;
pub struct Div;

pub struct Const {
    value: Value,
}

impl Const {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

pub struct Identifier {
    identifier: String,
}

impl Identifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

impl Operator for RootNode {
    fn precedence(&self) -> i32 {
        i32::min_value()
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, _arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        Err(Error::EvaluatedRootNode)
    }
}

impl Operator for Add {
    fn precedence(&self) -> i32 {
        95
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_number(&arguments[0])?;
        let b = expect_number(&arguments[1])?;

        Ok(Value::Number(a + b))
    }
}

impl Operator for Sub {
    fn precedence(&self) -> i32 {
        95
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_number(&arguments[0])?;
        let b = expect_number(&arguments[1])?;

        Ok(Value::Number(a - b))
    }
}

impl Operator for Mul {
    fn precedence(&self) -> i32 {
        100
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_number(&arguments[0])?;
        let b = expect_number(&arguments[1])?;

        Ok(Value::Number(a * b))
    }
}

impl Operator for Div {
    fn precedence(&self) -> i32 {
        100
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_number(&arguments[0])?;
        let b = expect_number(&arguments[1])?;

        Ok(Value::Number(a / b))
    }
}

impl Operator for Const {
    fn precedence(&self) -> i32 {
        200
    }

    fn argument_amount(&self) -> usize {
        0
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 0)?;

        Ok(self.value.clone())
    }
}

impl Operator for Identifier {
    fn precedence(&self) -> i32 {
        200
    }

    fn argument_amount(&self) -> usize {
        0
    }

    fn eval(&self, arguments: &[Value], configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 0)?;

        configuration.get_value(&self.identifier)
    }
}
