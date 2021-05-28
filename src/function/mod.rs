use std::fmt;

use crate::{error::EvalexprResult, value::Value};

pub(crate) mod builtin;

/// A user-defined function.
/// Functions can be used in expressions by storing them in a `Context`.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::new();
/// context.set_function("id".into(), Function::new(|argument| {
///     Ok(argument.clone())
/// })).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("id(4)", &context), Ok(Value::from(4)));
/// ```
#[derive(Clone)]
pub struct Function {
    function: fn(&Value) -> EvalexprResult<Value>,
}

impl Function {
    /// Creates a user-defined function.
    ///
    /// The `function` is a boxed function that takes a `Value` and returns a `EvalexprResult<Value, Error>`.
    pub fn new(function: fn(&Value) -> EvalexprResult<Value>) -> Self {
        Self { function }
    }

    pub(crate) fn call(&self, argument: &Value) -> EvalexprResult<Value> {
        (self.function)(argument)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Function {{ [...] }}")
    }
}

/// A trait to ensure a type is `Send` and `Sync`.
/// If implemented for a type, the crate will not compile if the type is not `Send` and `Sync`.
trait IsSendAndSync: Send + Sync {}

impl IsSendAndSync for Function {}
