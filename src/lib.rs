use configuration::EmptyConfiguration;
use error::Error;
use value::Value;

mod configuration;
mod error;
mod operator;
mod token;
mod tree;
mod value;

pub fn eval(string: &str) -> Result<Value, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string))?.eval(&EmptyConfiguration)
}

#[cfg(test)]
mod test {
    use crate::{eval, value::Value};

    #[test]
    fn test_simple_examples() {
        assert_eq!(eval("3"), Ok(Value::Number(3.0)));
        assert_eq!(eval("1+3"), Ok(Value::Number(4.0)));
        assert_eq!(eval("3+1"), Ok(Value::Number(4.0)));
        assert_eq!(eval("3-5"), Ok(Value::Number(-2.0)));
        assert_eq!(eval("5-3"), Ok(Value::Number(2.0)));
    }
}
