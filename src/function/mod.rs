
use std::fmt;
use serde_json::Value;
use error::Error;


/// Custom function
pub struct Function {
    /// Maximum number of arguments.
    pub max_args: Option<usize>,
    /// Minimum number of arguments.
    pub min_args: Option<usize>,
    /// Accept values and return a result which contains a value.
    pub compiled: Box<Fn(Vec<Value>) -> Result<Value, Error> + Sync + Send>,
}

impl Function {
    /// Create a function with a closure.
    pub fn new<F>(closure: F) -> Function
        where F: 'static + Fn(Vec<Value>) -> Result<Value, Error> + Sync + Send
    {
        Function {
            max_args: None,
            min_args: None,
            compiled: Box::new(closure),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Function {{ max_args: {:?}, min_args: {:?} }}",
               self.max_args,
               self.min_args)
    }
}
