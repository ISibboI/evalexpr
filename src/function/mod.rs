use error::{self, Error};
use value::Value;

pub mod builtin;

pub struct Function {
    argument_amount: Option<usize>,
    function: Box<Fn(&[Value]) -> Result<Value, Error>>,
}

impl Function {
    pub fn new(
        argument_amount: Option<usize>,
        function: Box<Fn(&[Value]) -> Result<Value, Error>>,
    ) -> Self {
        Self {
            argument_amount,
            function,
        }
    }

    pub fn call(&self, arguments: &[Value]) -> Result<Value, Error> {
        if let Some(argument_amount) = self.argument_amount {
            error::expect_function_argument_amount(arguments.len(), argument_amount)?;
        }

        (self.function)(arguments)
    }
}
