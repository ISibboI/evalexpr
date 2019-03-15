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
pub struct Braced;

pub struct Add;
pub struct Sub;
pub struct Neg;
pub struct Mul;
pub struct Div;
pub struct Mod;

pub struct Eq;
pub struct Neq;
pub struct Gt;
pub struct Lt;
pub struct Geq;
pub struct Leq;
pub struct And;
pub struct Or;
pub struct Not;

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
        200
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 1)?;
        Ok(arguments[0].clone())
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
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            Ok(Value::Int(
                arguments[0].as_int().unwrap() + arguments[1].as_int().unwrap(),
            ))
        } else {
            Ok(Value::Float(
                arguments[0].as_float().unwrap() + arguments[1].as_float().unwrap(),
            ))
        }
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
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            Ok(Value::Int(
                arguments[0].as_int().unwrap() - arguments[1].as_int().unwrap(),
            ))
        } else {
            Ok(Value::Float(
                arguments[0].as_float().unwrap() - arguments[1].as_float().unwrap(),
            ))
        }
    }
}

impl Operator for Neg {
    fn precedence(&self) -> i32 {
        110
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 1)?;
        expect_number(&arguments[0])?;

        if arguments[0].is_int() {
            Ok(Value::Int(-arguments[0].as_int().unwrap()))
        } else {
            Ok(Value::Float(-arguments[0].as_float().unwrap()))
        }
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
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            Ok(Value::Int(
                arguments[0].as_int().unwrap() * arguments[1].as_int().unwrap(),
            ))
        } else {
            Ok(Value::Float(
                arguments[0].as_float().unwrap() * arguments[1].as_float().unwrap(),
            ))
        }
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
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            Ok(Value::Int(
                arguments[0].as_int().unwrap() / arguments[1].as_int().unwrap(),
            ))
        } else {
            Ok(Value::Float(
                arguments[0].as_float().unwrap() / arguments[1].as_float().unwrap(),
            ))
        }
    }
}

impl Operator for Mod {
    fn precedence(&self) -> i32 {
        100
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            Ok(Value::Int(
                arguments[0].as_int().unwrap() % arguments[1].as_int().unwrap(),
            ))
        } else {
            Ok(Value::Float(
                arguments[0].as_float().unwrap() % arguments[1].as_float().unwrap(),
            ))
        }
    }
}

impl Operator for Eq {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;

        if arguments[0] == arguments[1] {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    }
}

impl Operator for Neq {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;

        if arguments[0] != arguments[1] {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    }
}

impl Operator for Gt {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            if arguments[0].as_int().unwrap() > arguments[1].as_int().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_float().unwrap() > arguments[1].as_float().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        }
    }
}

impl Operator for Lt {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            if arguments[0].as_int().unwrap() < arguments[1].as_int().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_float().unwrap() < arguments[1].as_float().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        }
    }
}

impl Operator for Geq {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            if arguments[0].as_int().unwrap() >= arguments[1].as_int().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_float().unwrap() >= arguments[1].as_float().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        }
    }
}

impl Operator for Leq {
    fn precedence(&self) -> i32 {
        80
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if arguments[0].is_int() && arguments[1].is_int() {
            if arguments[0].as_int().unwrap() <= arguments[1].as_int().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_float().unwrap() <= arguments[1].as_float().unwrap() {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        }
    }
}

impl Operator for And {
    fn precedence(&self) -> i32 {
        75
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_boolean(&arguments[0])?;
        let b = expect_boolean(&arguments[1])?;

        if a && b {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    }
}

impl Operator for Or {
    fn precedence(&self) -> i32 {
        70
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 2)?;
        let a = expect_boolean(&arguments[0])?;
        let b = expect_boolean(&arguments[1])?;

        if a || b {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    }
}

impl Operator for Not {
    fn precedence(&self) -> i32 {
        110
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _configuration: &Configuration) -> Result<Value, Error> {
        expect_argument_amount(arguments.len(), 1)?;
        let a = expect_boolean(&arguments[0])?;

        if !a {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
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
