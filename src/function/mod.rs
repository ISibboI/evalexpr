use error::{self, EvalexprError};
use value::Value;

pub(crate) mod builtin;

/// A user-defined function.
/// Functions can be used in expressions by storing them in a `Configuration`.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_function("id", Function::new(Some(1), Box::new(|arguments| {
///     Ok(arguments[0].clone())
/// })));
/// assert_eq!(eval_with_configuration("id(4)", &configuration), Ok(Value::from(4)));
/// ```
pub struct Function {
    argument_amount: Option<usize>,
    function: Box<Fn(&[Value]) -> Result<Value, EvalexprError>>,
}

impl Function {
    /// Creates a user-defined function.
    ///
    /// The `argument_amount` is the amount of arguments this function takes.
    /// It is verified before the actual function is executed, assuming it is not `None`.
    ///
    /// The `function` is a boxed function that takes a slice of values and returns a `Result<Value, Error>`.
    pub fn new(
        argument_amount: Option<usize>,
        function: Box<Fn(&[Value]) -> Result<Value, EvalexprError>>,
    ) -> Self {
        Self {
            argument_amount,
            function,
        }
    }

    pub(crate) fn call(&self, arguments: &[Value]) -> Result<Value, EvalexprError> {
        if let Some(argument_amount) = self.argument_amount {
            error::expect_function_argument_amount(arguments.len(), argument_amount)?;
        }

        (self.function)(arguments)
    }
}
