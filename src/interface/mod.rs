use Configuration;
use EmptyContext;
use EvalexprError;
use FloatType;
use IntType;
use Node;
use token;
use tree;
use Value;
use value::TupleType;

/// Evaluate the given expression string.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval(string: &str) -> Result<Value, EvalexprError> {
    eval_with_configuration(string, &EmptyContext)
}

/// Evaluate the given expression string with the given configuration.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut configuration = HashMapContext::new();
/// configuration.set_value("one", 1).unwrap(); // Do proper error handling here
/// configuration.set_value("two", 2).unwrap(); // Do proper error handling here
/// configuration.set_value("three", 3).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_configuration("one + two + three", &configuration), Ok(Value::from(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<Value, EvalexprError> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval_with_configuration(configuration)
}

/// Build the operator tree for the given expression string.
///
/// The operator tree can later on be evaluated directly.
/// This saves runtime if a single expression should be evaluated multiple times, for example with differing configurations.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let precomputed = build_operator_tree("one + two + three").unwrap(); // Do proper error handling here
///
/// let mut configuration = HashMapContext::new();
/// configuration.set_value("one", 1).unwrap(); // Do proper error handling here
/// configuration.set_value("two", 2).unwrap(); // Do proper error handling here
/// configuration.set_value("three", 3).unwrap(); // Do proper error handling here
///
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(6)));
///
/// configuration.set_value("three", 5).unwrap(); // Do proper error handling here
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(8)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn build_operator_tree(string: &str) -> Result<Node, EvalexprError> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)
}

/// Evaluate the given expression string into a string.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string(string: &str) -> Result<String, EvalexprError> {
    match eval(string) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(EvalexprError::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int(string: &str) -> Result<IntType, EvalexprError> {
    match eval(string) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(EvalexprError::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float(string: &str) -> Result<FloatType, EvalexprError> {
    match eval(string) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(EvalexprError::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean(string: &str) -> Result<bool, EvalexprError> {
    match eval(string) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(EvalexprError::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple(string: &str) -> Result<TupleType, EvalexprError> {
    match eval(string) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(EvalexprError::expected_tuple(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a string with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<String, EvalexprError> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(EvalexprError::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<IntType, EvalexprError> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(EvalexprError::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<FloatType, EvalexprError> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(EvalexprError::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<bool, EvalexprError> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(EvalexprError::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple with the given configuration.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<TupleType, EvalexprError> {
    match eval_with_configuration(string, configuration) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(EvalexprError::expected_tuple(value)),
        Err(error) => Err(error),
    }
}
