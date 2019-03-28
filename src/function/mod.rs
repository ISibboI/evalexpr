use std::fmt;

use error::{self, EvalexprResult};
use value::Value;

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
/// context.set_function("id".into(), Function::new(Some(1), Box::new(|arguments| {
///     Ok(arguments[0].clone())
/// }))).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("id(4)", &context), Ok(Value::from(4)));
/// ```
pub struct Function {
    argument_amount: Option<usize>,
    function: Box<Fn(&[Value]) -> EvalexprResult<Value>>,
}

impl Function {
    /// Creates a user-defined function.
    ///
    /// The `argument_amount` is the amount of arguments this function takes.
    /// It is verified before the actual function is executed, assuming it is not `None`.
    ///
    /// The `function` is a boxed function that takes a slice of values and returns a `EvalexprResult<Value, Error>`.
    pub fn new(
        argument_amount: Option<usize>,
        function: Box<Fn(&[Value]) -> EvalexprResult<Value>>,
    ) -> Self {
        Self {
            argument_amount,
            function,
        }
    }

    pub(crate) fn call(&self, arguments: &[Value]) -> EvalexprResult<Value> {
        if let Some(argument_amount) = self.argument_amount {
            error::expect_function_argument_amount(arguments.len(), argument_amount)?;
        }

        (self.function)(arguments)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Function {{ argument_amount: {:?}, function: [...] }}",
            self.argument_amount
        )
    }
}
