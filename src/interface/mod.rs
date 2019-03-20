use token;
use tree;
use Configuration;
use EmptyConfiguration;
use Error;
use Node;
use Value;

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
pub fn eval(string: &str) -> Result<Value, Error> {
    eval_with_configuration(string, &EmptyConfiguration)
}

/// Evaluate the given expression string with the given configuration.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_variable("one", 1);
/// configuration.insert_variable("two", 2);
/// configuration.insert_variable("three", 3);
/// assert_eq!(eval_with_configuration("one + two + three", &configuration), Ok(Value::from(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<Value, Error> {
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
/// let precomputed = build_operator_tree("one + two + three").unwrap();
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_variable("one", 1);
/// configuration.insert_variable("two", 2);
/// configuration.insert_variable("three", 3);
///
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(6)));
///
/// configuration.insert_variable("three", 5);
/// assert_eq!(precomputed.eval_with_configuration(&configuration), Ok(Value::from(8)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn build_operator_tree(string: &str) -> Result<Node, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)
}
