use std::fmt::{Debug, Display};

use function::builtin::builtin_function;

use crate::{context::Context, error::*, value::Value};

mod display;

pub trait Operator: Debug + Display {
    /// Returns the precedence of the operator.
    /// A high precedence means that the operator has priority to be deeper in the tree.
    // Make this a const fn once #57563 is resolved
    fn precedence(&self) -> i32;

    /// Returns true if chains of operators with the same precedence as this one should be evaluated left-to-right,
    /// and false if they should be evaluated right-to-left.
    /// Left-to-right chaining has priority if operators with different order but same precedence are chained.
    // Make this a const fn once #57563 is resolved
    fn is_left_to_right(&self) -> bool;

    /// True if this operator is a leaf, meaning it accepts no arguments.
    // Make this a const fn once #57563 is resolved
    fn is_leaf(&self) -> bool {
        self.argument_amount() == 0
    }

    /// Returns the amount of arguments required by this operator.
    // Make this a const fn once #57563 is resolved
    fn argument_amount(&self) -> usize;

    /// Evaluates the operator with the given arguments and context.
    fn eval(&self, arguments: &[Value], context: &Context) -> EvalexprResult<Value>;
}

#[derive(Debug)]
pub struct RootNode;

#[derive(Debug)]
pub struct Add;
#[derive(Debug)]
pub struct Sub;
#[derive(Debug)]
pub struct Neg;
#[derive(Debug)]
pub struct Mul;
#[derive(Debug)]
pub struct Div;
#[derive(Debug)]
pub struct Mod;
#[derive(Debug)]
pub struct Exp;

#[derive(Debug)]
pub struct Eq;
#[derive(Debug)]
pub struct Neq;
#[derive(Debug)]
pub struct Gt;
#[derive(Debug)]
pub struct Lt;
#[derive(Debug)]
pub struct Geq;
#[derive(Debug)]
pub struct Leq;
#[derive(Debug)]
pub struct And;
#[derive(Debug)]
pub struct Or;
#[derive(Debug)]
pub struct Not;

#[derive(Debug)]
pub struct Tuple;

#[derive(Debug)]
pub struct Const {
    value: Value,
}

impl Const {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct VariableIdentifier {
    identifier: String,
}

impl VariableIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

#[derive(Debug)]
pub struct FunctionIdentifier {
    identifier: String,
}

impl FunctionIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}

impl Operator for RootNode {
    fn precedence(&self) -> i32 {
        200
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 1)?;
        Ok(arguments[0].clone())
    }
}

impl Operator for Add {
    fn precedence(&self) -> i32 {
        95
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            let result = a.checked_add(b);
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::addition_error(
                    arguments[0].clone(),
                    arguments[1].clone(),
                ))
            }
        } else {
            Ok(Value::Float(
                arguments[0].as_number().unwrap() + arguments[1].as_number().unwrap(),
            ))
        }
    }
}

impl Operator for Sub {
    fn precedence(&self) -> i32 {
        95
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            let result = a.checked_sub(b);
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::subtraction_error(
                    arguments[0].clone(),
                    arguments[1].clone(),
                ))
            }
        } else {
            Ok(Value::Float(
                arguments[0].as_number().unwrap() - arguments[1].as_number().unwrap(),
            ))
        }
    }
}

impl Operator for Neg {
    fn precedence(&self) -> i32 {
        110
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 1)?;
        expect_number(&arguments[0])?;

        if let Ok(a) = arguments[0].as_int() {
            let result = a.checked_neg();
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::negation_error(arguments[0].clone()))
            }
        } else {
            Ok(Value::Float(-arguments[0].as_number().unwrap()))
        }
    }
}

impl Operator for Mul {
    fn precedence(&self) -> i32 {
        100
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            let result = a.checked_mul(b);
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::multiplication_error(
                    arguments[0].clone(),
                    arguments[1].clone(),
                ))
            }
        } else {
            Ok(Value::Float(
                arguments[0].as_number().unwrap() * arguments[1].as_number().unwrap(),
            ))
        }
    }
}

impl Operator for Div {
    fn precedence(&self) -> i32 {
        100
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            let result = a.checked_div(b);
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::division_error(
                    arguments[0].clone(),
                    arguments[1].clone(),
                ))
            }
        } else {
            Ok(Value::Float(
                arguments[0].as_number().unwrap() / arguments[1].as_number().unwrap(),
            ))
        }
    }
}

impl Operator for Mod {
    fn precedence(&self) -> i32 {
        100
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            let result = a.checked_rem(b);
            if let Some(result) = result {
                Ok(Value::Int(result))
            } else {
                Err(EvalexprError::modulation_error(
                    arguments[0].clone(),
                    arguments[1].clone(),
                ))
            }
        } else {
            Ok(Value::Float(
                arguments[0].as_number().unwrap() % arguments[1].as_number().unwrap(),
            ))
        }
    }
}

impl Operator for Exp {
    fn precedence(&self) -> i32 {
        120
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        Ok(Value::Float(
            arguments[0]
                .as_number()
                .unwrap()
                .powf(arguments[1].as_number().unwrap()),
        ))
    }
}

impl Operator for Eq {
    fn precedence(&self) -> i32 {
        80
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;

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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;

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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            if a > b {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_number().unwrap() > arguments[1].as_number().unwrap() {
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            if a < b {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_number().unwrap() < arguments[1].as_number().unwrap() {
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            if a >= b {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_number().unwrap() >= arguments[1].as_number().unwrap() {
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
        expect_number(&arguments[0])?;
        expect_number(&arguments[1])?;

        if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
            if a <= b {
                Ok(Value::Boolean(true))
            } else {
                Ok(Value::Boolean(false))
            }
        } else {
            if arguments[0].as_number().unwrap() <= arguments[1].as_number().unwrap() {
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 2)?;
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

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 1)?;
        let a = expect_boolean(&arguments[0])?;

        if !a {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    }
}

impl Operator for Tuple {
    fn precedence(&self) -> i32 {
        40
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        2
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        if let Value::Tuple(tuple) = &arguments[0] {
            let mut tuple = tuple.clone();
            if let Value::Tuple(tuple2) = &arguments[1] {
                tuple.extend(tuple2.iter().cloned());
            } else {
                tuple.push(arguments[1].clone());
            }
            Ok(Value::from(tuple))
        } else {
            if let Value::Tuple(tuple) = &arguments[1] {
                let mut tuple = tuple.clone();
                tuple.insert(0, arguments[0].clone());
                Ok(Value::from(tuple))
            } else {
                Ok(Value::from(vec![
                    arguments[0].clone(),
                    arguments[1].clone(),
                ]))
            }
        }
    }
}

impl Operator for Const {
    fn precedence(&self) -> i32 {
        200
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        0
    }

    fn eval(&self, arguments: &[Value], _context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 0)?;

        Ok(self.value.clone())
    }
}

impl Operator for VariableIdentifier {
    fn precedence(&self) -> i32 {
        200
    }

    fn is_left_to_right(&self) -> bool {
        true
    }

    fn argument_amount(&self) -> usize {
        0
    }

    fn eval(&self, _arguments: &[Value], context: &Context) -> EvalexprResult<Value> {
        if let Some(value) = context.get_value(&self.identifier).cloned() {
            Ok(value)
        } else {
            Err(EvalexprError::VariableIdentifierNotFound(
                self.identifier.clone(),
            ))
        }
    }
}

impl Operator for FunctionIdentifier {
    fn precedence(&self) -> i32 {
        190
    }

    fn is_left_to_right(&self) -> bool {
        false
    }

    fn argument_amount(&self) -> usize {
        1
    }

    fn eval(&self, arguments: &[Value], context: &Context) -> EvalexprResult<Value> {
        expect_operator_argument_amount(arguments.len(), 1)?;

        let arguments = if let Value::Tuple(arguments) = &arguments[0] {
            arguments
        } else {
            arguments
        };

        if let Some(function) = context.get_function(&self.identifier) {
            function.call(arguments)
        } else if let Some(builtin_function) = builtin_function(&self.identifier) {
            builtin_function.call(arguments)
        } else {
            Err(EvalexprError::FunctionIdentifierNotFound(
                self.identifier.clone(),
            ))
        }
    }
}
