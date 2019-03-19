use error::{self, Error};
use value::Value;

pub struct Function {
    argument_amount: usize,
    function: Box<Fn(&[Value]) -> Result<Value, Error>>,
}

impl Function {
    pub fn new(
        argument_amount: usize,
        function: Box<Fn(&[Value]) -> Result<Value, Error>>,
    ) -> Self {
        Self {
            argument_amount,
            function,
        }
    }

    pub fn call(&self, arguments: &[Value]) -> Result<Value, Error> {
        error::expect_function_argument_amount(arguments.len(), self.argument_amount)?;
        (self.function)(arguments)
    }
}
