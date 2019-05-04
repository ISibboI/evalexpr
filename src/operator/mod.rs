use function::builtin::builtin_function;

use crate::{context::Context, error::*, value::Value};

mod display;

#[derive(Debug, PartialEq)]
pub enum Operator {
    RootNode,

    Add,
    Sub,
    Neg,
    Mul,
    Div,
    Mod,
    Exp,

    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
    And,
    Or,
    Not,

    Tuple,
    Assign,

    Chain,

    Const { value: Value },
    VariableIdentifier { identifier: String },
    FunctionIdentifier { identifier: String },
}

impl Operator {
    pub(crate) fn value(value: Value) -> Self {
        Operator::Const { value }
    }

    pub(crate) fn variable_identifier(identifier: String) -> Self {
        Operator::VariableIdentifier { identifier }
    }

    pub(crate) fn function_identifier(identifier: String) -> Self {
        Operator::FunctionIdentifier { identifier }
    }

    /// Returns the precedence of the operator.
    /// A high precedence means that the operator has priority to be deeper in the tree.
    // Make this a const fn once #57563 is resolved
    pub(crate) fn precedence(&self) -> i32 {
        use crate::operator::Operator::*;
        match self {
            RootNode => 200,

            Add | Sub => 95,
            Neg => 110,
            Mul | Div | Mod => 100,
            Exp => 120,

            Eq | Neq | Gt | Lt | Geq | Leq => 80,
            And => 75,
            Or => 70,
            Not => 110,

            Tuple => 40,
            Assign => 50,

            Chain => 0,

            Const { value: _ } => 200,
            VariableIdentifier { identifier: _ } => 200,
            FunctionIdentifier { identifier: _ } => 190,
        }
    }

    /// Returns true if chains of operators with the same precedence as this one should be evaluated left-to-right,
    /// and false if they should be evaluated right-to-left.
    /// Left-to-right chaining has priority if operators with different order but same precedence are chained.
    // Make this a const fn once #57563 is resolved
    pub(crate) fn is_left_to_right(&self) -> bool {
        use crate::operator::Operator::*;
        match self {
            Assign => false,
            FunctionIdentifier { identifier: _ } => false,
            _ => true,
        }
    }

    /// Returns true if chains of this operator should be flattened into one operator with many arguments.
    // Make this a const fn once #57563 is resolved
    pub(crate) fn is_sequence(&self) -> bool {
        use crate::operator::Operator::*;
        match self {
            Tuple | Chain => true,
            _ => false,
        }
    }

    /// True if this operator is a leaf, meaning it accepts no arguments.
    // Make this a const fn once #57563 is resolved
    pub(crate) fn is_leaf(&self) -> bool {
        self.max_argument_amount() == Some(0)
    }

    /// Returns the maximum amount of arguments required by this operator.
    // Make this a const fn once #57563 is resolved
    pub(crate) fn max_argument_amount(&self) -> Option<usize> {
        use crate::operator::Operator::*;
        match self {
            Add | Sub | Mul | Div | Mod | Exp | Eq | Neq | Gt | Lt | Geq | Leq | And | Or
            | Assign => Some(2),
            Tuple | Chain => None,
            Not | Neg | RootNode => Some(1),
            Const { value: _ } => Some(0),
            VariableIdentifier { identifier: _ } => Some(0),
            FunctionIdentifier { identifier: _ } => Some(1),
        }
    }

    /// Evaluates the operator with the given arguments and context.
    pub(crate) fn eval(&self, arguments: &[Value], context: &dyn Context) -> EvalexprResult<Value> {
        use crate::operator::Operator::*;
        match self {
            RootNode => {
                if let Some(first) = arguments.first() {
                    Ok(first.clone())
                } else {
                    Ok(Value::Empty)
                }
            }
            Add => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                expect_number_or_string(&arguments[0])?;
                expect_number_or_string(&arguments[1])?;

                if let (Ok(a), Ok(b)) = (arguments[0].as_string(), arguments[1].as_string()) {
                    let mut result = String::with_capacity(a.len() + b.len());
                    result.push_str(&a);
                    result.push_str(&b);
                    Ok(Value::String(result))
                } else if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
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
            Sub => {
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
            Neg => {
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
            Mul => {
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
            Div => {
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
            Mod => {
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
            Exp => {
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
            Eq => {
                expect_operator_argument_amount(arguments.len(), 2)?;

                if arguments[0] == arguments[1] {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Neq => {
                expect_operator_argument_amount(arguments.len(), 2)?;

                if arguments[0] != arguments[1] {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Gt => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                expect_number_or_string(&arguments[0])?;
                expect_number_or_string(&arguments[1])?;

                if let (Ok(a), Ok(b)) = (arguments[0].as_string(), arguments[1].as_string()) {
                    if a > b {
                        Ok(Value::Boolean(true))
                    } else {
                        Ok(Value::Boolean(false))
                    }
                } else if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
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
            Lt => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                expect_number_or_string(&arguments[0])?;
                expect_number_or_string(&arguments[1])?;

                if let (Ok(a), Ok(b)) = (arguments[0].as_string(), arguments[1].as_string()) {
                    if a < b {
                        Ok(Value::Boolean(true))
                    } else {
                        Ok(Value::Boolean(false))
                    }
                } else if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
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
            Geq => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                expect_number_or_string(&arguments[0])?;
                expect_number_or_string(&arguments[1])?;

                if let (Ok(a), Ok(b)) = (arguments[0].as_string(), arguments[1].as_string()) {
                    if a >= b {
                        Ok(Value::Boolean(true))
                    } else {
                        Ok(Value::Boolean(false))
                    }
                } else if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
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
            Leq => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                expect_number_or_string(&arguments[0])?;
                expect_number_or_string(&arguments[1])?;

                if let (Ok(a), Ok(b)) = (arguments[0].as_string(), arguments[1].as_string()) {
                    if a <= b {
                        Ok(Value::Boolean(true))
                    } else {
                        Ok(Value::Boolean(false))
                    }
                } else if let (Ok(a), Ok(b)) = (arguments[0].as_int(), arguments[1].as_int()) {
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
            And => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                let a = expect_boolean(&arguments[0])?;
                let b = expect_boolean(&arguments[1])?;

                if a && b {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Or => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                let a = expect_boolean(&arguments[0])?;
                let b = expect_boolean(&arguments[1])?;

                if a || b {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Not => {
                expect_operator_argument_amount(arguments.len(), 1)?;
                let a = expect_boolean(&arguments[0])?;

                if !a {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Tuple => {
                Ok(Value::Tuple(arguments.into()))

                /*expect_operator_argument_amount(arguments.len(), 2)?;
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
                }*/
            }
            Assign => Err(EvalexprError::ContextNotManipulable),
            Chain => {
                if arguments.is_empty() {
                    return Err(EvalexprError::wrong_operator_argument_amount(0, 1));
                }

                Ok(arguments.last().cloned().unwrap_or(Value::Empty))
            }
            Const { value } => {
                expect_operator_argument_amount(arguments.len(), 0)?;

                Ok(value.clone())
            }
            VariableIdentifier { identifier } => {
                if let Some(value) = context.get_value(&identifier).cloned() {
                    Ok(value)
                } else {
                    Err(EvalexprError::VariableIdentifierNotFound(
                        identifier.clone(),
                    ))
                }
            }
            FunctionIdentifier { identifier } => {
                expect_operator_argument_amount(arguments.len(), 1)?;
                let arguments = &arguments[0];

                if let Some(function) = context.get_function(&identifier) {
                    function.call(arguments)
                } else if let Some(builtin_function) = builtin_function(&identifier) {
                    builtin_function.call(arguments)
                } else {
                    Err(EvalexprError::FunctionIdentifierNotFound(
                        identifier.clone(),
                    ))
                }
            }
        }
    }

    /// Evaluates the operator with the given arguments and mutable context.
    pub(crate) fn eval_mut(
        &self,
        arguments: &[Value],
        context: &mut dyn Context,
    ) -> EvalexprResult<Value> {
        use crate::operator::Operator::*;
        match self {
            Assign => {
                expect_operator_argument_amount(arguments.len(), 2)?;
                let target = expect_string(&arguments[0])?;
                context.set_value(target.into(), arguments[1].clone())?;

                Ok(Value::Empty)
            }
            _ => self.eval(arguments, context),
        }
    }
}
