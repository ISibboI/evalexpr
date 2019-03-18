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
        error::expect_argument_amount(self.argument_amount, arguments.len())?;
        (self.function)(arguments)
    }
}
