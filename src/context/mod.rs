//! A context defines methods to retrieve variable values and call functions for literals in an expression tree.
//! If mutable, it also allows to assign to variables.
//!
//! This crate implements two basic variants, the `EmptyContext`, that returns `None` for each identifier and cannot be manipulated, and the `HashMapContext`, that stores its mappings in hash maps.
//! The HashMapContext is type-safe and returns an error if the user tries to assign a value of a different type than before to an identifier.

use std::{borrow::Cow, collections::HashMap};
use thin_trait_object::thin_trait_object;
use crate::{
    function::Function,
    value::{value_type::ValueType, Value},
    EvalexprError, EvalexprResult,
};

mod predefined;



/// A context that allows to assign to variables.
pub trait ContextWithMutableVariables: Context {
    /// Sets the variable with the given identifier to the given value.
    fn set_value(&mut self, _identifier: String, _value: Value) -> EvalexprResult<()> {
        Err(EvalexprError::ContextNotMutable)
    }
}

/// A context that allows to assign to function identifiers.
pub trait ContextWithMutableFunctions: Context {
    /// Sets the function with the given identifier to the given function.
    fn set_function(&mut self, _identifier: String, _function: Function) -> EvalexprResult<()> {
        Err(EvalexprError::ContextNotMutable)
    }
}

/*/// A context that allows to retrieve functions programmatically.
pub trait GetFunctionContext: Context {
    /// Returns the function that is linked to the given identifier.
    ///
    /// This might not be possible for all functions, as some might be hard-coded.
    /// In this case, a special error variant should be returned (Not yet implemented).
    fn get_function(&self, identifier: &str) -> Option<&Function>;
}*/

/// A context that returns `None` for each identifier.
#[derive(Debug, Default)]
pub struct EmptyContext;

impl Context for EmptyContext {
    fn get_value(&self, _identifier: &str) -> Option<Cow<'_, Value>> {
        None
    }

    fn call_function(&self, identifier: &str, _argument: &Value) -> EvalexprResult<Value> {
        Err(EvalexprError::FunctionIdentifierNotFound(
            identifier.to_string(),
        ))
    }
}

/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
///
/// This context is type-safe, meaning that an identifier that is assigned a value of some type once cannot be assigned a value of another type.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct HashMapContext {
    variables: HashMap<String, Value>,
    #[cfg_attr(feature = "serde_support", serde(skip))]
    functions: HashMap<String, Function>,
}

impl HashMapContext {
    /// Constructs a `HashMapContext` with no mappings.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Context for HashMapContext {
    fn get_value(&self, identifier: &str) -> Option<Cow<'_, Value>> {
        self.variables.get(identifier).map(Cow::Borrowed)
    }

    fn call_function(&self, identifier: &str, argument: &Value) -> EvalexprResult<Value> {
        if let Some(function) = self.functions.get(identifier) {
            function.call(argument)
        } else {
            Err(EvalexprError::FunctionIdentifierNotFound(
                identifier.to_string(),
            ))
        }
    }
}

impl ContextWithMutableVariables for HashMapContext {
    fn set_value(&mut self, identifier: String, value: Value) -> EvalexprResult<()> {
        if let Some(existing_value) = self.variables.get_mut(&identifier) {
            if ValueType::from(&existing_value) == ValueType::from(&value) {
                *existing_value = value;
                return Ok(());
            } else {
                return Err(EvalexprError::expected_type(existing_value, value));
            }
        }

        // Implicit else, because `self.variables` and `identifier` are not unborrowed in else
        self.variables.insert(identifier, value);
        Ok(())
    }
}

impl ContextWithMutableFunctions for HashMapContext {
    fn set_function(&mut self, identifier: String, function: Function) -> EvalexprResult<()> {
        self.functions.insert(identifier, function);
        Ok(())
    }
}

pub trait Context {
/// A context defines methods to retrieve variable values and call functions for literals in an expression tree.
    fn get_value(&self, identifier: &str) -> Option<Cow<'_, Value>>;
/// Retrieves the value of the given identifier.
    fn call_function(&self, idt: &str, argument: &Value) -> EvalexprResult<Value>;
}

#[thin_trait_object]
pub trait OperatorRowTrait {
    fn get_value(&self, identifier: &str) -> EvalexprResult<Value>;
    fn set_value(&mut self, identifier: &str, value: Value) -> EvalexprResult<()>;
    fn get_value_for_column(&self, col: usize) -> EvalexprResult<Value>;
    fn set_value_for_column(&mut self, col: usize, value: Value) -> EvalexprResult<()>;
    fn set_row(&mut self, row: usize);
    fn call_function(&self, idt: &str, argument: &Value) -> EvalexprResult<Value>;
    fn has_changes(&self) -> bool;
    fn get_dirty_flags(&self) -> EvalexprResult<Vec<usize>>;
}
#[thin_trait_object]
pub trait OperatorSchemaTrait {
    fn get_ordered_column_names(&self) -> EvalexprResult<Vec<String>>;
    fn get_column_name_for_column(&self, column: usize) -> EvalexprResult<String>;
    fn get_index_for_column(&self, column: &str) -> EvalexprResult<usize>;
}

/// This macro provides a convenient syntax for creating a static context.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let ctx = evalexpr::context_map! {
///     "x" => 8,
///     "f" => Function::new(|_| Ok(42.into()))
/// }.unwrap(); // Do proper error handling here
///
/// assert_eq!(eval_with_context("x + f()", &ctx), Ok(50.into()));
/// ```
#[macro_export]
macro_rules! context_map {
    // Termination (allow missing comma at the end of the argument list)
    ( ($ctx:expr) $k:expr => Function::new($($v:tt)*) ) =>
        { $crate::context_map!(($ctx) $k => Function::new($($v)*),) };
    ( ($ctx:expr) $k:expr => $v:expr ) =>
        { $crate::context_map!(($ctx) $k => $v,)  };
    // Termination
    ( ($ctx:expr) ) => { Ok(()) };

    // The user has to specify a literal 'Function::new' in order to create a function
    ( ($ctx:expr) $k:expr => Function::new($($v:tt)*) , $($tt:tt)*) => {{
        $crate::ContextWithMutableFunctions::set_function($ctx, $k.into(), $crate::Function::new($($v)*))
            .and($crate::context_map!(($ctx) $($tt)*))
    }};
    // add a value, and chain the eventual error with the ones in the next values
    ( ($ctx:expr) $k:expr => $v:expr , $($tt:tt)*) => {{
        $crate::ContextWithMutableVariables::set_value($ctx, $k.into(), $v.into())
            .and($crate::context_map!(($ctx) $($tt)*))
    }};

    // Create a context, then recurse to add the values in it
    ( $($tt:tt)* ) => {{
        let mut context = $crate::HashMapContext::new();
        $crate::context_map!((&mut context) $($tt)*)
            .map(|_| context)
    }};
}
