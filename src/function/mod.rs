use std::fmt;

use crate::error::EvalexprResult;
use crate::value::Value;

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
/// context.set_function("id".into(), Function::new(Box::new(|argument| {
///     Ok(argument.clone())
/// }))).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("id(4)", &context), Ok(Value::from(4)));
/// ```
pub struct Function {
    function: Box<dyn Fn(&Value) -> EvalexprResult<Value>>,
}

impl Function {
    /// Creates a user-defined function.
    ///
    /// The `function` is a boxed function that takes a `Value` and returns a `EvalexprResult<Value, Error>`.
    pub fn new(function: Box<dyn Fn(&Value) -> EvalexprResult<Value>>) -> Self {
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
