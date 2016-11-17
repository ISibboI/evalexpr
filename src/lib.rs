//! Expression evaluator.
//!
//! Supported operators: `!` `!=` `""` `''` `()` `[]` `,` `>` `<` `>=` `<=`
//! `==` `+` `-` `*` `/` `%` `&&` `||` `n..m`.
//!
//! Built-in functions: `min()` `max()` `is_empty()`.
//!
//! # Examples
//!
//! You can do mathematical calculations with supported operators:
//!
//! ```
//! use eval::{eval, to_value};
//!
//! assert_eq!(eval("1 + 2 + 3"), Ok(to_value(6)));
//! assert_eq!(eval("2 * 2 + 3"), Ok(to_value(7)));
//! assert_eq!(eval("2 / 2 + 3"), Ok(to_value(4.0)));
//! assert_eq!(eval("2 / 2 + 3 / 3"), Ok(to_value(2.0)));
//! ```
//!
//! You can eval with context:
//!
//! ```
//! use eval::{eval_with_context, Context, to_value};
//!
//! let mut context = Context::new();
//! context.insert("foo".to_owned(), to_value(true));
//! context.insert("bar".to_owned(), to_value(true));
//! assert_eq!(eval_with_context("foo == bar", &context), Ok(to_value(true)));
//! ```
//!
//! You can eval with functions:
//!
//! ```
//! use eval::{eval_with_functions, Functions, Function, to_value};
//!
//! let mut functions = Functions::new();
//! functions.insert("say_hello".to_owned(), Function::new(|_| Ok(to_value("Hello world!"))));
//! assert_eq!(eval_with_functions("say_hello()", &functions), Ok(to_value("Hello world!")));
//! ```
//!
//! You can create an array with `[]`:
//!
//! ```
//! use eval::{eval, to_value};
//!
//! assert_eq!(eval("[1, 2, 3, 4, 5]"), Ok(to_value(vec![1, 2, 3, 4, 5])));
//!
//! ```
//!
//! You can create an integer array with `n..m`:
//!
//! ```
//! use eval::{eval, to_value};
//!
//! assert_eq!(eval("0..5"), Ok(to_value(vec![0, 1, 2, 3, 4])));
//!
//! ```
//!
#![recursion_limit="100"]
#![deny(missing_docs)]
#![feature(proc_macro, test)]
extern crate test;

#[macro_use]
extern crate quick_error;
extern crate serde_json;


mod math;
mod function;
mod operator;
mod node;
mod expression;
mod error;
mod builtin;


pub use serde_json::{Value, to_value};
pub use error::Error;
pub use function::Function;

use std::collections::HashMap;
use expression::Expression;
use builtin::BuiltIn;

type ContextsRef<'a> = &'a [Context];

/// Eval context.
pub type Context = HashMap<String, Value>;
/// Eval contexts. The value of the last context is searched first.
pub type Contexts = Vec<Context>;
/// Eval functions.
pub type Functions = HashMap<String, Function>;

/// Evaluates the value of an expression.
pub fn eval(expr: &str) -> Result<Value, Error> {
    Expression::new(expr)?.compile()(&Contexts::new(), &BuiltIn::new(), &Functions::new())
}

/// Evaluates the value of an expression with the given context.
pub fn eval_with_context(expr: &str, context: &Context) -> Result<Value, Error> {
    let mut contexts = Contexts::new();
    contexts.push(context.clone());
    eval_with_contexts(expr, &contexts)
}

/// Evaluates the value of an expression with the given contexts.<br>
/// The value of the last context is searched first.
pub fn eval_with_contexts(expr: &str, contexts: ContextsRef) -> Result<Value, Error> {
    Expression::new(expr)?.compile()(contexts, &BuiltIn::new(), &Functions::new())
}

/// Evaluates the value of an expression with the given functions.
pub fn eval_with_functions(expr: &str, functions: &Functions) -> Result<Value, Error> {
    Expression::new(expr)?.compile()(&Contexts::new(), &BuiltIn::new(), functions)
}

/// Evaluates the value of an expression with the given context and functions.
pub fn eval_with_context_and_functions(expr: &str,
                                       context: &Context,
                                       functions: &Functions)
                                       -> Result<Value, Error> {
    let mut contexts = Contexts::new();
    contexts.push(context.clone());
    eval_with_contexts_and_functions(expr, &contexts, functions)
}

/// Evaluates the value of an expression with the given contexts and functions.<br>
/// The value of the last context is searched first.
pub fn eval_with_contexts_and_functions(expr: &str,
                                        contexts: ContextsRef,
                                        functions: &Functions)
                                        -> Result<Value, Error> {
    Expression::new(expr)?.compile()(contexts, &BuiltIn::new(), functions)
}


#[cfg(test)]
mod tests {
    use test;
    use serde_json::to_value;
    use expression::Expression;
    use error::Error;
    use Context;
    use eval;
    use eval_with_context;
    use eval_with_functions;
    use {Function, Functions};

    #[test]
    fn test_add() {
        assert_eq!(eval("2 + 3"), Ok(to_value(5)));
    }

    #[test]
    fn test_brackets_add() {
        assert_eq!(eval("(2 + 3) + (3 + 5)"), Ok(to_value(13)));
    }

    #[test]
    fn test_brackets_float_add() {
        assert_eq!(eval("(2 + 3.2) + (3 + 5)"), Ok(to_value(13.2)));
    }

    #[test]
    fn test_brackets_float_mul() {
        assert_eq!(eval("(2 + 3.2) * 5"), Ok(to_value(26.0)));
    }

    #[test]
    fn test_brackets_sub() {
        assert_eq!(eval("(4 - 3) * 5"), Ok(to_value(5)));
    }

    #[test]
    fn test_useless_brackets() {
        assert_eq!(eval("2 + 3 + (5)"), Ok(to_value(10)));
    }

    #[test]
    fn test_error_brackets_not_with_function() {
        assert_eq!(eval("5 + ()"), Err(Error::BracketNotWithFunction));
    }

    #[test]
    fn test_deep_brackets() {
        assert_eq!(eval("(2 + (3 + 4) + (6 + (6 + 7)) + 5)"), Ok(to_value(33)));
    }

    #[test]
    fn test_brackets_div() {
        assert_eq!(eval("(4 / (2 + 2)) * 5"), Ok(to_value(5.0)));
    }

    #[test]
    fn test_min() {
        assert_eq!(eval("min(30, 5, 245, 20)"), Ok(to_value(5)));
    }

    #[test]
    fn test_min_brackets() {
        assert_eq!(eval("(min(30, 5, 245, 20) * 10 + (5 + 5) * 5)"),
                   Ok(to_value(100)));
    }

    #[test]
    fn test_min_and_mul() {
        assert_eq!(eval("min(30, 5, 245, 20) * 10"), Ok(to_value(50)));
    }

    #[test]
    fn test_max() {
        assert_eq!(eval("max(30, 5, 245, 20)"), Ok(to_value(245)));
    }

    #[test]
    fn test_max_brackets() {
        assert_eq!(eval("(max(30, 5, 245, 20) * 10 + (5 + 5) * 5)"),
                   Ok(to_value(2500)));
    }

    #[test]
    fn test_max_and_mul() {
        assert_eq!(eval("max(30, 5, 245, 20) * 10"), Ok(to_value(2450)));
    }

    #[test]
    fn test_brackets_1() {
        assert_eq!(eval("(5) + (min(3, 4, 5)) + 20"), Ok(to_value(28)));
    }

    #[test]
    fn test_brackets_2() {
        assert_eq!(eval("(((5) / 5))"), Ok(to_value(1.0)));
    }

    #[test]
    fn test_string_add() {
        assert_eq!(eval(r#""Hello"+", world!""#), Ok(to_value("Hello, world!")));
    }

    #[test]
    fn test_equal() {
        assert_eq!(eval("1 == 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(eval("1 != 2"), Ok(to_value(true)));
    }

    #[test]
    fn test_multiple_equal() {
        assert_eq!(eval("(1 == 2) == (2 == 3)"), Ok(to_value(true)));
    }

    #[test]
    fn test_multiple_not_equal() {
        assert_eq!(eval("(1 != 2) == (2 != 3)"), Ok(to_value(true)));
    }

    #[test]
    fn test_greater_than() {
        assert_eq!(eval("1 > 2"), Ok(to_value(false)));
        assert_eq!(eval("2 > 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_less_than() {
        assert_eq!(eval("2 < 1"), Ok(to_value(false)));
        assert_eq!(eval("1 < 2"), Ok(to_value(true)));
    }

    #[test]
    fn test_greater_and_less() {
        assert_eq!(eval("(2 > 1) == (1 < 2)"), Ok(to_value(true)));
    }

    #[test]
    fn test_ge() {
        assert_eq!(eval("2 >= 1"), Ok(to_value(true)));
        assert_eq!(eval("2 >= 2"), Ok(to_value(true)));
        assert_eq!(eval("2 >= 3"), Ok(to_value(false)));
    }

    #[test]
    fn test_le() {
        assert_eq!(eval("2 <= 1"), Ok(to_value(false)));
        assert_eq!(eval("2 <= 2"), Ok(to_value(true)));
        assert_eq!(eval("2 <= 3"), Ok(to_value(true)));
    }

    #[test]
    fn test_quotes() {
        assert_eq!(eval(r#""1><2" + "3<>4""#), Ok(to_value("1><23<>4")));
        assert_eq!(eval(r#""1==2" + "3--4""#), Ok(to_value("1==23--4")));
        assert_eq!(eval(r#""1!=2" + "3>>4""#), Ok(to_value("1!=23>>4")));
        assert_eq!(eval(r#""><1!=2" + "3>>4""#), Ok(to_value("><1!=23>>4")));
    }

    #[test]
    fn test_single_quote() {
        assert_eq!(eval(r#"'1><2' + '3<>4'"#), Ok(to_value("1><23<>4")));
        assert_eq!(eval(r#"'1==2' + '3--4'"#), Ok(to_value("1==23--4")));
        assert_eq!(eval(r#"'1!=2' + '3>>4'"#), Ok(to_value("1!=23>>4")));
        assert_eq!(eval(r#"'!=1<>2' + '3>>4'"#), Ok(to_value("!=1<>23>>4")));
    }

    #[test]
    fn test_single_and_double_quote() {
        assert_eq!(eval(r#"' """" ' + ' """" '"#),
                   Ok(to_value(r#" """"  """" "#)));
    }

    #[test]
    fn test_double_and_single_quote() {
        assert_eq!(eval(r#"" '''' " + " '''' ""#),
                   Ok(to_value(r#" ''''  '''' "#)));
    }

    #[test]
    fn test_array() {
        assert_eq!(eval("[1, 2, 3, 4]"), Ok(to_value(vec![1, 2, 3, 4])));
        assert_eq!(eval("array(1, 2, 3, 4)"), Ok(to_value(vec![1, 2, 3, 4])));
    }

    #[test]
    fn test_array_ident() {
        assert_eq!(eval("0..5"), Ok(to_value(vec![0, 1, 2, 3, 4])));
    }

    #[test]
    fn test_array_ident_and_min() {
        assert_eq!(eval("min(0..5)"), Ok(to_value(0)));
    }

    #[test]
    fn test_rem_1() {
        assert_eq!(eval("2 % 2"), Ok(to_value(0)));
    }

    #[test]
    fn test_rem_2() {
        assert_eq!(eval("5 % 56 % 5"), Ok(to_value(0)));
    }

    #[test]
    fn test_rem_3() {
        assert_eq!(eval("5.5 % 23"), Ok(to_value(5.5)));
    }

    #[test]
    fn test_rem_4() {
        assert_eq!(eval("23 % 5.5"), Ok(to_value(1.0)));
    }

    #[test]
    fn test_and_1() {
        assert_eq!(eval("3 > 2 && 2 > 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_and_2() {
        assert_eq!(eval("3 == 2 && 2 == 1"), Ok(to_value(false)));
    }

    #[test]
    fn test_and_3() {
        assert_eq!(eval("3 > 2 && 2 == 1"), Ok(to_value(false)));
    }

    #[test]
    fn test_or_1() {
        assert_eq!(eval("3 > 2 || 2 > 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_or_2() {
        assert_eq!(eval("3 < 2 || 2 < 1"), Ok(to_value(false)));
    }

    #[test]
    fn test_or_3() {
        assert_eq!(eval("3 > 2 || 2 < 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_or_4() {
        assert_eq!(eval("3 < 2 || 2 > 1"), Ok(to_value(true)));
    }

    #[test]
    fn test_not() {
        assert_eq!(eval("!false"), Ok(to_value(true)));
        assert_eq!(eval("!true"), Ok(to_value(false)));
        assert_eq!(eval("!(1 != 2)"), Ok(to_value(false)));
        assert_eq!(eval("!(1 == 2)"), Ok(to_value(true)));
        assert_eq!(eval("!(1 == 2) == true"), Ok(to_value(true)));
    }

    #[test]
    fn test_not_and_brackets() {
        assert_eq!(eval("(!(1 == 2)) == true"), Ok(to_value(true)));
    }

    #[test]
    fn test_buildin_is_empty() {
        let mut context = Context::new();
        context.insert("array".to_owned(), to_value(Vec::<String>::new()));
        assert_eq!(eval_with_context("is_empty(array)", &context),
                   Ok(to_value(true)));
    }

    #[test]
    fn test_buildin_min() {
        let mut context = Context::new();
        context.insert("array".to_owned(), to_value(vec![23, 34, 45, 2]));
        assert_eq!(eval_with_context("min(array)", &context), Ok(to_value(2)));
    }

    #[test]
    fn test_custom_function() {
        let mut functions = Functions::new();
        functions.insert("output".to_owned(),
                         Function::new(|_| Ok(to_value("This is custom function's output"))));
        assert_eq!(eval_with_functions("output()", &functions),
                   Ok(to_value("This is custom function's output")));
    }

    #[test]
    fn test_error_start_with_non_value_operator() {
        let mut expr = Expression {
            raw: "+ + 5".to_owned(),
            pos: Vec::new(),
            operators: Vec::new(),
            node: None,
        };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();

        assert_eq!(expr.parse_node(), Err(Error::StartWithNonValueOperator));
    }

    #[test]
    fn test_error_duplicate_operator() {
        let mut expr = Expression { raw: "5 + + 5".to_owned(), ..Default::default() };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();

        assert_eq!(expr.parse_node(), Err(Error::DuplicateOperatorNode));
    }

    #[test]
    fn test_error_duplicate_value() {
        let mut expr = Expression { raw: "2 + 6 5".to_owned(), ..Default::default() };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();

        assert_eq!(expr.parse_node(), Err(Error::DuplicateValueNode));
    }

    #[test]
    fn test_error_unpaired_brackets() {
        let mut expr = Expression { raw: "(2 + 3)) * 5".to_owned(), ..Default::default() };

        expr.parse_pos().unwrap();

        assert_eq!(expr.parse_operators(), Err(Error::UnpairedBrackets));
    }

    #[test]
    fn test_error_comma() {
        let mut expr = Expression { raw: ", 2 + 5".to_owned(), ..Default::default() };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();

        assert_eq!(expr.parse_node(), Err(Error::CommaNotWithFunction));
    }


    #[bench]
    fn bench_deep_brackets(b: &mut test::Bencher) {
        b.iter(|| eval("(2 + (3 + 4) + (6 + (6 + 7)) + 5)"));
    }

    #[bench]
    fn bench_parse_pos(b: &mut test::Bencher) {
        let mut expr = Expression {
            raw: "(2 + (3 + 4) + (6 + (6 + 7)) + 5)".to_owned(),
            ..Default::default()
        };

        b.iter(|| expr.parse_pos().unwrap());
    }

    #[bench]
    fn bench_parse_operators(b: &mut test::Bencher) {
        let mut expr = Expression {
            raw: "(2 + (3 + 4) + (6 + (6 + 7)) + 5)".to_owned(),
            ..Default::default()
        };

        expr.parse_pos().unwrap();
        b.iter(|| expr.parse_operators().unwrap());
    }

    #[bench]
    fn bench_parse_nodes(b: &mut test::Bencher) {
        let mut expr = Expression {
            raw: "(2 + (3 + 4) + (6 + (6 + 7)) + 5)".to_owned(),
            ..Default::default()
        };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();
        b.iter(|| expr.parse_node().unwrap());
    }

    #[bench]
    fn bench_compile(b: &mut test::Bencher) {
        let mut expr = Expression {
            raw: "(2 + (3 + 4) + (6 + (6 + 7)) + 5)".to_owned(),
            ..Default::default()
        };

        expr.parse_pos().unwrap();
        expr.parse_operators().unwrap();
        expr.parse_node().unwrap();
        b.iter(|| expr.compile());
    }

    #[bench]
    fn bench_eval(b: &mut test::Bencher) {
        b.iter(|| eval("(2 + (3 + 4) + (6 + (6 + 7)) + 5)"));
    }
}
