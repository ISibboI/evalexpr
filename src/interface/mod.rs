use crate::{
    error::EvalexprResultValue,
    token, tree,
    value::{
        numeric_types::{default_numeric_types::DefaultNumericTypes, EvalexprNumericTypes},
        TupleType,
    },
    Context, ContextWithMutableVariables, EmptyType, EvalexprError, EvalexprResult, HashMapContext,
    Node, Operator, Value, EMPTY_VALUE,
};

/// Evaluate the given expression string.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// assert_eq!(eval("1 + 2 + 3"), Ok(Value::from_int(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval(string: &str) -> EvalexprResultValue {
    eval_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string with the given context.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::<DefaultNumericTypes>::new();
/// context.set_value("one".into(), Value::from_int(1)).unwrap(); // Do proper error handling here
/// context.set_value("two".into(), Value::from_int(2)).unwrap(); // Do proper error handling here
/// context.set_value("three".into(), Value::from_int(3)).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("one + two + three", &context), Ok(Value::from_int(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResultValue<C::NumericTypes> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval_with_context(context)
}

/// Evaluate the given expression string with the given mutable context.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::<DefaultNumericTypes>::new();
/// context.set_value("one".into(), Value::from_int(1)).unwrap(); // Do proper error handling here
/// context.set_value("two".into(), Value::from_int(2)).unwrap(); // Do proper error handling here
/// context.set_value("three".into(), Value::from_int(3)).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context_mut("one + two + three", &mut context), Ok(Value::from_int(6)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResultValue<C::NumericTypes> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval_with_context_mut(context)
}

/// Build the operator tree for the given expression string.
///
/// The operator tree can later on be evaluated directly.
/// This saves runtime if a single expression should be evaluated multiple times, for example with differing contexts.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let precomputed = build_operator_tree("one + two + three").unwrap(); // Do proper error handling here
///
/// let mut context = HashMapContext::<DefaultNumericTypes>::new();
/// context.set_value("one".into(), Value::from_int(1)).unwrap(); // Do proper error handling here
/// context.set_value("two".into(), Value::from_int(2)).unwrap(); // Do proper error handling here
/// context.set_value("three".into(), Value::from_int(3)).unwrap(); // Do proper error handling here
///
/// assert_eq!(precomputed.eval_with_context(&context), Ok(Value::from_int(6)));
///
/// context.set_value("three".into(), Value::from_int(5)).unwrap(); // Do proper error handling here
/// assert_eq!(precomputed.eval_with_context(&context), Ok(Value::from_int(8)));
/// ```
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn build_operator_tree<NumericTypes: EvalexprNumericTypes>(
    string: &str,
) -> EvalexprResult<Node<NumericTypes>, NumericTypes> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)
}

/// Evaluate the given expression string into a string.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string(string: &str) -> EvalexprResult<String> {
    eval_string_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into an integer.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int(
    string: &str,
) -> EvalexprResult<<DefaultNumericTypes as EvalexprNumericTypes>::Int> {
    eval_int_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float(
    string: &str,
) -> EvalexprResult<<DefaultNumericTypes as EvalexprNumericTypes>::Float> {
    eval_float_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into a float.
/// If the result of the expression is an integer, it is silently converted into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_number(
    string: &str,
) -> EvalexprResult<<DefaultNumericTypes as EvalexprNumericTypes>::Float> {
    eval_number_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into a boolean.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean(string: &str) -> EvalexprResult<bool> {
    eval_boolean_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into a tuple.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple(string: &str) -> EvalexprResult<TupleType> {
    eval_tuple_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into an empty value.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_empty(string: &str) -> EvalexprResult<EmptyType> {
    eval_empty_with_context_mut(string, &mut HashMapContext::<DefaultNumericTypes>::new())
}

/// Evaluate the given expression string into a string with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<String, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(EvalexprError::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Int, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(EvalexprError::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Float, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(EvalexprError::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given context.
/// If the result of the expression is an integer, it is silently converted into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_number_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Float, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(Value::Int(int)) => Ok(<C::NumericTypes as EvalexprNumericTypes>::int_as_float(
            &int,
        )),
        Ok(value) => Err(EvalexprError::expected_number(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<bool, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(EvalexprError::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<TupleType<C::NumericTypes>, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(EvalexprError::expected_tuple(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an empty value with the given context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_empty_with_context<C: Context>(
    string: &str,
    context: &C,
) -> EvalexprResult<EmptyType, C::NumericTypes> {
    match eval_with_context(string, context) {
        Ok(Value::Empty) => Ok(EMPTY_VALUE),
        Ok(value) => Err(EvalexprError::expected_empty(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a string with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_string_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<String, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::String(string)) => Ok(string),
        Ok(value) => Err(EvalexprError::expected_string(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an integer with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_int_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Int, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Int(int)) => Ok(int),
        Ok(value) => Err(EvalexprError::expected_int(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_float_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Float, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(value) => Err(EvalexprError::expected_float(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a float with the given mutable context.
/// If the result of the expression is an integer, it is silently converted into a float.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_number_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<<C::NumericTypes as EvalexprNumericTypes>::Float, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Float(float)) => Ok(float),
        Ok(Value::Int(int)) => Ok(<C::NumericTypes as EvalexprNumericTypes>::int_as_float(
            &int,
        )),
        Ok(value) => Err(EvalexprError::expected_number(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a boolean with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_boolean_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<bool, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Boolean(boolean)) => Ok(boolean),
        Ok(value) => Err(EvalexprError::expected_boolean(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into a tuple with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_tuple_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<TupleType<C::NumericTypes>, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Tuple(tuple)) => Ok(tuple),
        Ok(value) => Err(EvalexprError::expected_tuple(value)),
        Err(error) => Err(error),
    }
}

/// Evaluate the given expression string into an empty value with the given mutable context.
///
/// *See the [crate doc](index.html) for more examples and explanations of the expression format.*
pub fn eval_empty_with_context_mut<C: ContextWithMutableVariables>(
    string: &str,
    context: &mut C,
) -> EvalexprResult<EmptyType, C::NumericTypes> {
    match eval_with_context_mut(string, context) {
        Ok(Value::Empty) => Ok(EMPTY_VALUE),
        Ok(value) => Err(EvalexprError::expected_empty(value)),
        Err(error) => Err(error),
    }
}

/// Combined two parsed expression trees around an operator
/// ```
///use evalexpr::{build_operator_tree, combine_trees, eval, Operator};
///let expr_1 = build_operator_tree("42/2").unwrap();
///let expr_2 = build_operator_tree("34*5").unwrap();
///
///let combined = combine_trees(expr_1, expr_2, Operator::Add).unwrap();
///assert_eq!(combined.eval(), eval("(42/2)+(34*5)"))
/// ```
pub fn combine_trees<NumericTypes: EvalexprNumericTypes>(
    a: Node<NumericTypes>,
    b: Node<NumericTypes>,
    operator: Operator<NumericTypes>,
) -> EvalexprResult<Node<NumericTypes>, NumericTypes> {
    tree::combine_trees(a, b, operator)
}
