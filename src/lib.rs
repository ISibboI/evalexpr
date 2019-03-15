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
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval(&EmptyConfiguration)
}

#[cfg(test)]
mod test {
    use crate::{eval, value::Value};
    use error::Error;

    #[test]
    fn test_unary_examples() {
        assert_eq!(eval("3"), Ok(Value::Int(3)));
        assert_eq!(eval("3.3"), Ok(Value::Float(3.3)));
        assert_eq!(eval("true"), Ok(Value::Boolean(true)));
        assert_eq!(eval("false"), Ok(Value::Boolean(false)));
        assert_eq!(eval("blub"), Err(Error::IdentifierNotFound));
        assert_eq!(eval("-3"), Ok(Value::Int(-3)));
        assert_eq!(eval("-3.6"), Ok(Value::Float(-3.6)));
        assert_eq!(eval("----3"), Ok(Value::Int(3)));
    }

    #[test]
    fn test_binary_examples() {
        assert_eq!(eval("1+3"), Ok(Value::Int(4)));
        assert_eq!(eval("3+1"), Ok(Value::Int(4)));
        assert_eq!(eval("3-5"), Ok(Value::Int(-2)));
        assert_eq!(eval("5-3"), Ok(Value::Int(2)));
        assert_eq!(eval("5 / 4"), Ok(Value::Int(1)));
        assert_eq!(eval("5 *3"), Ok(Value::Int(15)));
        assert_eq!(eval("1.0+3"), Ok(Value::Float(4.0)));
        assert_eq!(eval("3.0+1"), Ok(Value::Float(4.0)));
        assert_eq!(eval("3-5.0"), Ok(Value::Float(-2.0)));
        assert_eq!(eval("5-3.0"), Ok(Value::Float(2.0)));
        assert_eq!(eval("5 / 4.0"), Ok(Value::Float(1.25)));
        assert_eq!(eval("5.0 *3"), Ok(Value::Float(15.0)));
        assert_eq!(eval("5.0 *-3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 *- 3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 * -3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 * - 3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("-5.0 *-3"), Ok(Value::Float(15.0)));
        assert_eq!(eval("3+-1"), Ok(Value::Int(2)));
        assert_eq!(eval("-3-5"), Ok(Value::Int(-8)));
        assert_eq!(eval("-5--3"), Ok(Value::Int(-2)));
    }

    #[test]
    fn test_arithmetic_precedence_examples() {
        assert_eq!(eval("1+3-2"), Ok(Value::Int(2)));
        assert_eq!(eval("3+1*5"), Ok(Value::Int(8)));
        assert_eq!(eval("2*3-5"), Ok(Value::Int(1)));
        assert_eq!(eval("5-3/3"), Ok(Value::Int(4)));
        assert_eq!(eval("5 / 4*2"), Ok(Value::Int(2)));
        assert_eq!(eval("1-5 *3/15"), Ok(Value::Int(0)));
        assert_eq!(eval("15/7/2.0"), Ok(Value::Float(1.0)));
        assert_eq!(eval("15.0/7/2"), Ok(Value::Float(15.0 / 7.0 / 2.0)));
        assert_eq!(eval("15.0/-7/2"), Ok(Value::Float(15.0 / -7.0 / 2.0)));
        assert_eq!(eval("-15.0/7/2"), Ok(Value::Float(-15.0 / 7.0 / 2.0)));
        assert_eq!(eval("-15.0/7/-2"), Ok(Value::Float(-15.0 / 7.0 / -2.0)));
    }

    #[test]
    fn test_braced_examples() {
        assert_eq!(eval("(1)"), Ok(Value::Int(1)));
        assert_eq!(eval("( 1.0 )"), Ok(Value::Float(1.0)));
        assert_eq!(eval("( true)"), Ok(Value::Boolean(true)));
        assert_eq!(eval("( -1 )"), Ok(Value::Int(-1)));
        assert_eq!(eval("-(1)"), Ok(Value::Int(-1)));
        assert_eq!(eval("-(1 + 3) * 7"), Ok(Value::Int(-28)));
        assert_eq!(eval("(1 * 1) - 3"), Ok(Value::Int(-2)));
        assert_eq!(eval("4 / (2 * 2)"), Ok(Value::Int(1)));
        assert_eq!(eval("7/(7/(7/(7/(7/(7)))))"), Ok(Value::Int(1)));
    }

    #[test]
    fn test_mod_examples() {
        assert_eq!(eval("1 % 4"), Ok(Value::Int(1)));
        assert_eq!(eval("6 % 4"), Ok(Value::Int(2)));
        assert_eq!(eval("1 % 4 + 2"), Ok(Value::Int(3)));
    }

    #[test]
    fn test_boolean_examples() {
        assert_eq!(eval("true && false"), Ok(Value::Boolean(false)));
        assert_eq!(eval("true && false || true && true"), Ok(Value::Boolean(true)));
        assert_eq!(eval("5 > 4 && 1 <= 1"), Ok(Value::Boolean(true)));
        assert_eq!(eval("5.0 <= 4.9 || !(4 > 3.5)"), Ok(Value::Boolean(false)));
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            eval("-true"),
            Err(Error::expected_number(Value::Boolean(true)))
        );
        assert_eq!(
            eval("1-true"),
            Err(Error::expected_number(Value::Boolean(true)))
        );
        assert_eq!(eval("true-"), Err(Error::wrong_argument_amount(1, 2)));
        assert_eq!(eval("!(()true)"), Err(Error::AppendedToLeafNode));
    }
}
