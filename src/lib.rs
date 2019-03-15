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
    use error::Error;

    #[test]
    fn test_unary_examples() {
        assert_eq!(eval("3"), Ok(Value::Int(3)));
        assert_eq!(eval("true"), Ok(Value::Boolean(true)));
        assert_eq!(eval("false"), Ok(Value::Boolean(false)));
        assert_eq!(eval("blub"), Err(Error::IdentifierNotFound));
    }

    #[test]
    fn test_binary_examples() {
        assert_eq!(eval("1+3"), Ok(Value::Int(4)));
        assert_eq!(eval("3+1"), Ok(Value::Int(4)));
        assert_eq!(eval("3-5"), Ok(Value::Int(-2)));
        assert_eq!(eval("5-3"), Ok(Value::Int(2)));
        assert_eq!(eval("5 / 4"), Ok(Value::Int(1)));
        assert_eq!(eval("5 *3"), Ok(Value::Int(15)));
    }

    #[test]
    fn test_arithmetic_precedence_examples() {
        assert_eq!(eval("1+3-2"), Ok(Value::Int(2)));
        assert_eq!(eval("3+1*5"), Ok(Value::Int(8)));
        assert_eq!(eval("2*3-5"), Ok(Value::Int(1)));
        assert_eq!(eval("5-3/3"), Ok(Value::Int(4)));
        assert_eq!(eval("5 / 4*2"), Ok(Value::Int(2)));
        assert_eq!(eval("1-5 *3/15"), Ok(Value::Int(0)));
    }
}
