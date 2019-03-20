//!
//! ## Quickstart
//!
//! Add `evalexpr` as dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! evalexpr = "0.5"
//! ```
//!
//! Add the `extern crate` definition to your `main.rs` or `lib.rs`:
//!
//! ```rust
//! extern crate evalexpr;
//! ```
//!
//! Then you can use `evalexpr` to evaluate expressions like this:
//!
//! ```rust
//! use evalexpr::*;
//!
//! assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
//! assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
//! assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
//! assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
//! ```
//!
//! And you can use variables and functions in expressions like this:
//!
//! ```rust
//! use evalexpr::*;
//! use evalexpr::error::expect_number;
//!
//! let mut configuration = HashMapConfiguration::new();
//! configuration.insert_variable("five", 5);
//! configuration.insert_variable("twelve", 12);
//! configuration.insert_function("f", Function::new(Some(1) /* argument amount */, Box::new(|arguments| {
//!     if let Value::Int(int) = arguments[0] {
//!         Ok(Value::Int(int / 2))
//!     } else if let Value::Float(float) = arguments[0] {
//!         Ok(Value::Float(float / 2.0))
//!     } else {
//!         Err(Error::expected_number(arguments[0].clone()))
//!     }
//! })));
//! configuration.insert_function("avg", Function::new(Some(2) /* argument amount */, Box::new(|arguments| {
//!     expect_number(&arguments[0])?;
//!     expect_number(&arguments[1])?;
//!
//!     if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
//!         Ok(Value::Int((a + b) / 2))
//!     } else {
//!         Ok(Value::Float((arguments[0].as_float()? + arguments[1].as_float()?) / 2.0))
//!     }
//! })));
//!
//! assert_eq!(eval_with_configuration("five + 8 > f(twelve)", &configuration), Ok(Value::from(true)));
//! assert_eq!(eval_with_configuration("avg(2, 4) == 3", &configuration), Ok(Value::from(true)));
//! ```
//!
//! You can also precompile expressions like this:
//!
//! ```rust
//! use evalexpr::*;
//!
//! let precompiled = build_operator_tree("a * b - c > 5").unwrap();
//!
//! let mut configuration = HashMapConfiguration::new();
//! configuration.insert_variable("a", 6);
//! configuration.insert_variable("b", 2);
//! configuration.insert_variable("c", 3);
//! assert_eq!(precompiled.eval_with_configuration(&configuration), Ok(Value::from(true)));
//!
//! configuration.insert_variable("c", 8);
//! assert_eq!(precompiled.eval_with_configuration(&configuration), Ok(Value::from(false)));
//! ```
//!
//! ## Features
//!
//! ### Operators
//!
//! This crate offers a set of binary and unary operators for building expressions.
//! Operators have a precedence to determine their order of evaluation.
//! The precedence should resemble that of most common programming languages, especially Rust.
//! The precedence of variables and values is 200, and the precedence of function literals is 190.
//!
//! Supported binary operators:
//!
//! | Operator | Precedence | Description |   | Operator | Precedence | Description |
//! |----------|------------|-------------|---|----------|------------|-------------|
//! | + | 95 | Sum | | < | 80 | Lower than |
//! | - | 95 | Difference | | \> | 80 | Greater than |
//! | * | 100 | Product | | <= | 80 | Lower than or equal |
//! | / | 100 | Division | | \>= | 80 | Greater than or equal |
//! | % | 100 | Modulo | | == | 80 | Equal |
//! | ^ | 120 | Exponentiation | | != | 80 | Not equal |
//! | && | 75 | Logical and | | , | 40 | Aggregation |
//! | &#124;&#124; | 70 | Logical or | | | | |
//!
//! Supported unary operators:
//!
//! | Operator | Precedence | Description |
//! |----------|------------|-------------|
//! | - | 110 | Negation |
//! | ! | 110 | Logical not |
//!
//! #### The Aggregation Operator
//!
//! The aggregation operator aggregates two values into a tuple.
//! If one of the values is a tuple already, the resulting tuple will be flattened.
//! Example:
//!
//! ```rust
//! use evalexpr::*;
//!
//! assert_eq!(eval("1, 2, 3"), Ok(Value::from(vec![Value::from(1), Value::from(2), Value::from(3)])));
//! ```
//!
//! ### Builtin Functions
//!
//! This crate offers a set of builtin functions.
//!
//! | Identifier | Argument Amount | Description |
//! |------------|-----------------|-------------|
//! | min | >= 1 | Returns the minimum of the arguments |
//! | max | >= 1 | Returns the maximum of the arguments |
//!
//! The `min` and `max` functions can deal with a mixture of integer and floating point arguments.
//! They return the result as the type it was passed into the function.
//!
//! ### Values
//!
//! Operators take values as arguments and produce values as results.
//! Values can be boolean, integer or floating point numbers.
//! Strings are supported as well, but there are no operations defined for them yet.
//! Values are denoted as displayed in the following table.
//!
//! | Value type | Example |
//! |------------|---------|
//! | `Value::Boolean` | `true`, `false` |
//! | `Value::Int` | `3`, `-9`, `0`, `135412` |
//! | `Value::Float` | `3.`, `.35`, `1.00`, `0.5`, `123.554` |
//!
//! Integers are internally represented as `i64`, and floating point numbers are represented as `f64`.
//! Operators that take numbers as arguments can either take integers or floating point numbers.
//! If one of the arguments is a floating point number, all others are converted to floating point numbers as well, and the resulting value is a floating point number as well.
//! Otherwise, the result is an integer.
//! An exception to this is the exponentiation operator that always returns a floating point number.
//!
//! Values have a precedence of 200.
//!
//! ### Variables
//!
//! This crate allows to compile parameterizable formulas by using variables.
//! A variable is a literal in the formula, that does not contain whitespace or can be parsed as value.
//! The user needs to provide bindings to the variables for evaluation.
//! This is done with the `Configuration` trait.
//! Two structs implementing this trait are predefined.
//! There is `EmptyConfiguration`, that returns `None` for each request, and `HashMapConfiguration`, that stores mappings from literals to variables in a hash map.
//!
//! Variables do not have fixed types in the expression itself, but aer typed by the configuration.
//! The `Configuration` trait contains a function that takes a string literal and returns a `Value` enum.
//! The variant of this enum decides the type on evaluation.
//!
//! Variables have a precedence of 200.
//!
//! ### User-Defined Functions
//!
//! This crate also allows to define arbitrary functions to be used in parsed expressions.
//! A function is defined as a `Function` instance.
//! It contains two properties, the `argument_amount` and the `function`.
//! The `function` is a boxed `Fn(&[Value]) -> Result<Value, Error>`.
//! The `argument_amount` determines the length of the slice that is passed to `function` if it is `Some(_)`, otherwise the function is defined to take an arbitrary amount of arguments.
//! It is verified on execution by the crate and does not need to be verified by the `function`.
//!
//! Functions with no arguments are not allowed.
//! Use variables instead.
//!
//! Be aware that functions need to verify the types of values that are passed to them.
//! The `error` module contains some shortcuts for verification, and error types for passing a wrong value type.
//! Also, most numeric functions need to differentiate between being called with integers or floating point numbers, and act accordingly.
//!
//! Functions are identified by literals, like variables as well.
//! A literal identifies a function, if it is followed by an opening brace `(`, another literal, or a value.
//!
//! Same as variables, function bindings are provided by the user via a `Configuration`.
//! Functions have a precedence of 190.
//!
//! ### Examplary variables and functions in expressions:
//!
//! | Expression | Valid? | Explanation |
//! |------------|--------|-------------|
//! | `a` | yes | |
//! | `abc` | yes | |
//! | `a<b` | no | Expression is interpreted as variable `a`, operator `<` and variable `b` |
//! | `a b` | no | Expression is interpreted as function `a` applied to argument `b` |
//! | `123` | no | Expression is interpreted as `Value::Int` |
//! | `true` | no | Expression is interpreted as `Value::Bool` |
//! | `.34` | no | Expression is interpreted as `Value::Float` |
//!
//! ## License
//!
//! This crate is primarily distributed under the terms of the MIT license.
//! See [LICENSE](LICENSE) for details.
//!

#![warn(missing_docs)]

extern crate core;

mod configuration;
pub mod error;
mod function;
mod operator;
mod token;
mod tree;
mod value;

// Exports

pub use configuration::{Configuration, EmptyConfiguration, HashMapConfiguration};
pub use error::Error;
pub use function::Function;
pub use tree::Node;
pub use value::Value;

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

#[cfg(test)]
mod test {
    use crate::{eval, value::Value};
    use configuration::HashMapConfiguration;
    use error::{expect_number, Error};
    use eval_with_configuration;
    use value::IntType;
    use Function;

    #[test]
    fn test_unary_examples() {
        assert_eq!(eval("3"), Ok(Value::Int(3)));
        assert_eq!(eval("3.3"), Ok(Value::Float(3.3)));
        assert_eq!(eval("true"), Ok(Value::Boolean(true)));
        assert_eq!(eval("false"), Ok(Value::Boolean(false)));
        assert_eq!(
            eval("blub"),
            Err(Error::VariableIdentifierNotFound("blub".to_string()))
        );
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
    fn test_pow_examples() {
        assert_eq!(eval("1 ^ 4"), Ok(Value::Float(1.0)));
        assert_eq!(eval("6 ^ 4"), Ok(Value::Float(6.0f64.powf(4.0))));
        assert_eq!(eval("1 ^ 4 + 2"), Ok(Value::Float(3.0)));
        assert_eq!(eval("2 ^ (4 + 2)"), Ok(Value::Float(64.0)));
    }

    #[test]
    fn test_boolean_examples() {
        assert_eq!(eval("true && false"), Ok(Value::Boolean(false)));
        assert_eq!(
            eval("true && false || true && true"),
            Ok(Value::Boolean(true))
        );
        assert_eq!(eval("5 > 4 && 1 <= 1"), Ok(Value::Boolean(true)));
        assert_eq!(eval("5.0 <= 4.9 || !(4 > 3.5)"), Ok(Value::Boolean(false)));
    }

    #[test]
    fn test_with_configuration() {
        let mut configuration = HashMapConfiguration::new();
        configuration.insert_variable("tr".to_string(), Value::Boolean(true));
        configuration.insert_variable("fa".to_string(), Value::Boolean(false));
        configuration.insert_variable("five".to_string(), Value::Int(5));
        configuration.insert_variable("six".to_string(), Value::Int(6));
        configuration.insert_variable("half".to_string(), Value::Float(0.5));
        configuration.insert_variable("zero".to_string(), Value::Int(0));

        assert_eq!(
            eval_with_configuration("tr", &configuration),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            eval_with_configuration("fa", &configuration),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            eval_with_configuration("tr && false", &configuration),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            eval_with_configuration("five + six", &configuration),
            Ok(Value::Int(11))
        );
        assert_eq!(
            eval_with_configuration("five * half", &configuration),
            Ok(Value::Float(2.5))
        );
        assert_eq!(
            eval_with_configuration("five < six && true", &configuration),
            Ok(Value::Boolean(true))
        );
    }

    #[test]
    fn test_functions() {
        let mut configuration = HashMapConfiguration::new();
        configuration.insert_function(
            "sub2".to_string(),
            Function::new(
                Some(1),
                Box::new(|arguments| {
                    if let Value::Int(int) = arguments[0] {
                        Ok(Value::Int(int - 2))
                    } else if let Value::Float(float) = arguments[0] {
                        Ok(Value::Float(float - 2.0))
                    } else {
                        Err(Error::expected_number(arguments[0].clone()))
                    }
                }),
            ),
        );
        configuration.insert_variable("five".to_string(), Value::Int(5));

        assert_eq!(
            eval_with_configuration("sub2 5", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(5)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2 five", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(five)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(3) + five", &configuration),
            Ok(Value::Int(6))
        );
    }

    #[test]
    fn test_n_ary_functions() {
        let mut configuration = HashMapConfiguration::new();
        configuration.insert_function(
            "sub2",
            Function::new(
                Some(1),
                Box::new(|arguments| {
                    if let Value::Int(int) = arguments[0] {
                        Ok(Value::Int(int - 2))
                    } else if let Value::Float(float) = arguments[0] {
                        Ok(Value::Float(float - 2.0))
                    } else {
                        Err(Error::expected_number(arguments[0].clone()))
                    }
                }),
            ),
        );
        configuration.insert_function(
            "avg",
            Function::new(
                Some(2),
                Box::new(|arguments| {
                    expect_number(&arguments[0])?;
                    expect_number(&arguments[1])?;

                    if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                        Ok(Value::Int((a + b) / 2))
                    } else {
                        Ok(Value::Float(
                            (arguments[0].as_float()? + arguments[1].as_float()?) / 2.0,
                        ))
                    }
                }),
            ),
        );
        configuration.insert_function(
            "muladd",
            Function::new(
                Some(3),
                Box::new(|arguments| {
                    expect_number(&arguments[0])?;
                    expect_number(&arguments[1])?;
                    expect_number(&arguments[2])?;

                    if let (Value::Int(a), Value::Int(b), Value::Int(c)) =
                        (&arguments[0], &arguments[1], &arguments[2])
                    {
                        Ok(Value::Int(a * b + c))
                    } else {
                        Ok(Value::Float(
                            arguments[0].as_float()? * arguments[1].as_float()?
                                + arguments[2].as_float()?,
                        ))
                    }
                }),
            ),
        );
        configuration.insert_function(
            "count",
            Function::new(
                None,
                Box::new(|arguments| Ok(Value::Int(arguments.len() as IntType))),
            ),
        );
        configuration.insert_variable("five".to_string(), Value::Int(5));

        assert_eq!(
            eval_with_configuration("avg(7, 5)", &configuration),
            Ok(Value::Int(6))
        );
        assert_eq!(
            eval_with_configuration("avg(sub2 5, 5)", &configuration),
            Ok(Value::Int(4))
        );
        assert_eq!(
            eval_with_configuration("sub2(avg(3, 6))", &configuration),
            Ok(Value::Int(2))
        );
        assert_eq!(
            eval_with_configuration("sub2 avg(3, 6)", &configuration),
            Ok(Value::Int(2))
        );
        assert_eq!(
            eval_with_configuration("muladd(3, 6, -4)", &configuration),
            Ok(Value::Int(14))
        );
        assert_eq!(
            eval_with_configuration("count()", &configuration),
            Err(Error::wrong_operator_argument_amount(0, 1))
        );
        assert_eq!(
            eval_with_configuration("count(3, 5.5, 2)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("count 5", &configuration),
            Ok(Value::Int(1))
        );

        assert_eq!(
            eval_with_configuration("min(4.0, 3)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("max(4.0, 3)", &configuration),
            Ok(Value::Float(4.0))
        );
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
        assert_eq!(
            eval("true-"),
            Err(Error::wrong_operator_argument_amount(1, 2))
        );
        assert_eq!(eval("!(()true)"), Err(Error::AppendedToLeafNode));
    }

    #[test]
    fn test_no_panic() {
        assert!(eval(&format!(
            "{} + {}",
            IntType::max_value(),
            IntType::max_value()
        ))
        .is_err());
        assert!(eval(&format!(
            "-{} - {}",
            IntType::max_value(),
            IntType::max_value()
        ))
        .is_err());
        assert!(eval(&format!("-(-{} - 1)", IntType::max_value())).is_err());
        assert!(eval(&format!(
            "{} * {}",
            IntType::max_value(),
            IntType::max_value()
        ))
        .is_err());
        assert!(eval(&format!("{} / {}", IntType::max_value(), 0)).is_err());
        assert!(eval(&format!("{} % {}", IntType::max_value(), 0)).is_err());
        assert!(eval(&format!(
            "{} ^ {}",
            IntType::max_value(),
            IntType::max_value()
        ))
        .is_ok());
    }
}
