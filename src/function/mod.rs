use std::fmt;

use crate::{
    error::EvalexprResultValue,
    value::{
        numeric_types::{default_numeric_types::DefaultNumericTypes, EvalexprNumericTypes},
        Value,
    },
};

pub(crate) mod builtin;

/// A helper trait to enable cloning through `Fn` trait objects.
trait ClonableFn<NumericTypes: EvalexprNumericTypes = DefaultNumericTypes>
where
    Self: Fn(&Value<NumericTypes>) -> EvalexprResultValue<NumericTypes>,
    Self: Send + Sync + 'static,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn<NumericTypes>>;
}

impl<F, NumericTypes: EvalexprNumericTypes> ClonableFn<NumericTypes> for F
where
    F: Fn(&Value<NumericTypes>) -> EvalexprResultValue<NumericTypes>,
    F: Send + Sync + 'static,
    F: Clone,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn<NumericTypes>> {
        Box::new(self.clone()) as _
    }
}

/// A user-defined function.
/// Functions can be used in expressions by storing them in a `Context`.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::<DefaultNumericTypes>::new();
/// context.set_function("id".into(), Function::new(|argument| {
///     Ok(argument.clone())
/// })).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("id(4)", &context), Ok(Value::from_int(4)));
/// ```
pub struct Function<NumericTypes: EvalexprNumericTypes> {
    function: Box<dyn ClonableFn<NumericTypes>>,
}

impl<NumericTypes: EvalexprNumericTypes> Clone for Function<NumericTypes> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.dyn_clone(),
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> Function<NumericTypes> {
    /// Creates a user-defined function.
    ///
    /// The `function` is boxed for storage.
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(&Value<NumericTypes>) -> EvalexprResultValue<NumericTypes>,
        F: Send + Sync + 'static,
        F: Clone,
    {
        Self {
            function: Box::new(function) as _,
        }
    }

    pub(crate) fn call(&self, argument: &Value<NumericTypes>) -> EvalexprResultValue<NumericTypes> {
        (self.function)(argument)
    }
}

impl<NumericTypes: EvalexprNumericTypes> fmt::Debug for Function<NumericTypes> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Function {{ [...] }}")
    }
}

/// A trait to ensure a type is `Send` and `Sync`.
/// If implemented for a type, the crate will not compile if the type is not `Send` and `Sync`.
#[allow(dead_code)]
#[doc(hidden)]
trait IsSendAndSync: Send + Sync {}

impl<NumericTypes: EvalexprNumericTypes> IsSendAndSync for Function<NumericTypes> {}
