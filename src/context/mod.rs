//! A context defines methods to retrieve variable values and call functions for literals in an expression tree.
//! If mutable, it also allows to assign to variables.
//!
//! This crate implements two basic variants, the `EmptyContext`, that returns `None` for each identifier and cannot be manipulated, and the `HashMapContext`, that stores its mappings in hash maps.
//! The HashMapContext is type-safe and returns an error if the user tries to assign a value of a different type than before to an identifier.

use std::{collections::HashMap, iter, marker::PhantomData};

use crate::{
    error::EvalexprResultValue,
    function::Function,
    value::{
        numeric_types::{default_numeric_types::DefaultNumericTypes, EvalexprNumericTypes},
        value_type::ValueType,
        Value,
    },
    EvalexprError, EvalexprResult,
};

mod predefined;

/// An immutable context.
pub trait Context {
    /// The numeric types used for evaluation.
    type NumericTypes: EvalexprNumericTypes;

    /// Returns the value that is linked to the given identifier.
    fn get_value(&self, identifier: &str) -> Option<&Value<Self::NumericTypes>>;

    /// Calls the function that is linked to the given identifier with the given argument.
    /// If no function with the given identifier is found, this method returns `EvalexprError::FunctionIdentifierNotFound`.
    fn call_function(
        &self,
        identifier: &str,
        argument: &Value<Self::NumericTypes>,
    ) -> EvalexprResultValue<Self::NumericTypes>;

    /// Checks if builtin functions are disabled.
    fn are_builtin_functions_disabled(&self) -> bool;

    /// Disables builtin functions if `disabled` is `true`, and enables them otherwise.
    /// If the context does not support enabling or disabling builtin functions, an error is returned.
    fn set_builtin_functions_disabled(
        &mut self,
        disabled: bool,
    ) -> EvalexprResult<(), Self::NumericTypes>;
}

/// A context that allows to assign to variables.
pub trait ContextWithMutableVariables: Context {
    /// Sets the variable with the given identifier to the given value.
    fn set_value(
        &mut self,
        _identifier: String,
        _value: Value<Self::NumericTypes>,
    ) -> EvalexprResult<(), Self::NumericTypes> {
        Err(EvalexprError::ContextNotMutable)
    }
    /// Removes the variable with the given identifier from the context.
    fn remove_value(
        &mut self,
        _identifier: &str
    ) -> EvalexprResult<Option<Value<Self::NumericTypes>>, Self::NumericTypes> {
        Err(EvalexprError::ContextNotMutable)
    }
}

/// A context that allows to assign to function identifiers.
pub trait ContextWithMutableFunctions: Context {
    /// Sets the function with the given identifier to the given function.
    fn set_function(
        &mut self,
        _identifier: String,
        _function: Function<Self::NumericTypes>,
    ) -> EvalexprResult<(), Self::NumericTypes> {
        Err(EvalexprError::ContextNotMutable)
    }
}

/// A context that allows to iterate over its variable names with their values.
pub trait IterateVariablesContext: Context {
    /// The iterator type for iterating over variable name-value pairs.
    type VariableIterator<'a>: Iterator<Item = (String, Value<Self::NumericTypes>)>
    where
        Self: 'a;
    /// The iterator type for iterating over variable names.
    type VariableNameIterator<'a>: Iterator<Item = String>
    where
        Self: 'a;

    /// Returns an iterator over pairs of variable names and values.
    fn iter_variables(&self) -> Self::VariableIterator<'_>;

    /// Returns an iterator over variable names.
    fn iter_variable_names(&self) -> Self::VariableNameIterator<'_>;
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
/// Builtin functions are disabled and cannot be enabled.
#[derive(Debug)]
pub struct EmptyContext<NumericTypes>(PhantomData<NumericTypes>);

impl<NumericTypes: EvalexprNumericTypes> Context for EmptyContext<NumericTypes> {
    type NumericTypes = NumericTypes;

    fn get_value(&self, _identifier: &str) -> Option<&Value<Self::NumericTypes>> {
        None
    }

    fn call_function(
        &self,
        identifier: &str,
        _argument: &Value<Self::NumericTypes>,
    ) -> EvalexprResultValue<Self::NumericTypes> {
        Err(EvalexprError::FunctionIdentifierNotFound(
            identifier.to_string(),
        ))
    }

    /// Builtin functions are always disabled for `EmptyContext`.
    fn are_builtin_functions_disabled(&self) -> bool {
        true
    }

    /// Builtin functions can't be enabled for `EmptyContext`.
    fn set_builtin_functions_disabled(
        &mut self,
        disabled: bool,
    ) -> EvalexprResult<(), Self::NumericTypes> {
        if disabled {
            Ok(())
        } else {
            Err(EvalexprError::BuiltinFunctionsCannotBeEnabled)
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> IterateVariablesContext for EmptyContext<NumericTypes> {
    type VariableIterator<'a>
        = iter::Empty<(String, Value<Self::NumericTypes>)>
    where
        Self: 'a;
    type VariableNameIterator<'a>
        = iter::Empty<String>
    where
        Self: 'a;

    fn iter_variables(&self) -> Self::VariableIterator<'_> {
        iter::empty()
    }

    fn iter_variable_names(&self) -> Self::VariableNameIterator<'_> {
        iter::empty()
    }
}

impl<NumericTypes> Default for EmptyContext<NumericTypes> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// A context that returns `None` for each identifier.
/// Builtin functions are enabled and cannot be disabled.
#[derive(Debug)]
pub struct EmptyContextWithBuiltinFunctions<NumericTypes>(PhantomData<NumericTypes>);

impl<NumericTypes: EvalexprNumericTypes> Context
    for EmptyContextWithBuiltinFunctions<NumericTypes>
{
    type NumericTypes = NumericTypes;

    fn get_value(&self, _identifier: &str) -> Option<&Value<Self::NumericTypes>> {
        None
    }

    fn call_function(
        &self,
        identifier: &str,
        _argument: &Value<Self::NumericTypes>,
    ) -> EvalexprResultValue<Self::NumericTypes> {
        Err(EvalexprError::FunctionIdentifierNotFound(
            identifier.to_string(),
        ))
    }

    /// Builtin functions are always enabled for EmptyContextWithBuiltinFunctions.
    fn are_builtin_functions_disabled(&self) -> bool {
        false
    }

    /// Builtin functions can't be disabled for EmptyContextWithBuiltinFunctions.
    fn set_builtin_functions_disabled(
        &mut self,
        disabled: bool,
    ) -> EvalexprResult<(), Self::NumericTypes> {
        if disabled {
            Err(EvalexprError::BuiltinFunctionsCannotBeDisabled)
        } else {
            Ok(())
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> IterateVariablesContext
    for EmptyContextWithBuiltinFunctions<NumericTypes>
{
    type VariableIterator<'a>
        = iter::Empty<(String, Value<Self::NumericTypes>)>
    where
        Self: 'a;
    type VariableNameIterator<'a>
        = iter::Empty<String>
    where
        Self: 'a;

    fn iter_variables(&self) -> Self::VariableIterator<'_> {
        iter::empty()
    }

    fn iter_variable_names(&self) -> Self::VariableNameIterator<'_> {
        iter::empty()
    }
}

impl<NumericTypes> Default for EmptyContextWithBuiltinFunctions<NumericTypes> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
///
/// This context is type-safe, meaning that an identifier that is assigned a value of some type once cannot be assigned a value of another type.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HashMapContext<NumericTypes: EvalexprNumericTypes = DefaultNumericTypes> {
    variables: HashMap<String, Value<NumericTypes>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    functions: HashMap<String, Function<NumericTypes>>,

    /// True if builtin functions are disabled.
    without_builtin_functions: bool,
}

impl<NumericTypes: EvalexprNumericTypes> HashMapContext<NumericTypes> {
    /// Constructs a `HashMapContext` with no mappings.
    pub fn new() -> Self {
        Default::default()
    }

    /// Removes all variables from the context.
    /// This allows to reuse the context without allocating a new HashMap.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use evalexpr::*;
    ///
    /// let mut context = HashMapContext::<DefaultNumericTypes>::new();
    /// context.set_value("abc".into(), "def".into()).unwrap();
    /// assert_eq!(context.get_value("abc"), Some(&("def".into())));
    /// context.clear_variables();
    /// assert_eq!(context.get_value("abc"), None);
    /// ```
    pub fn clear_variables(&mut self) {
        self.variables.clear()
    }

    /// Removes all functions from the context.
    /// This allows to reuse the context without allocating a new HashMap.
    pub fn clear_functions(&mut self) {
        self.functions.clear()
    }

    /// Removes all variables and functions from the context.
    /// This allows to reuse the context without allocating a new HashMap.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use evalexpr::*;
    ///
    /// let mut context = HashMapContext::<DefaultNumericTypes>::new();
    /// context.set_value("abc".into(), "def".into()).unwrap();
    /// assert_eq!(context.get_value("abc"), Some(&("def".into())));
    /// context.clear();
    /// assert_eq!(context.get_value("abc"), None);
    /// ```
    pub fn clear(&mut self) {
        self.clear_variables();
        self.clear_functions();
    }
}

impl<NumericTypes: EvalexprNumericTypes> Context for HashMapContext<NumericTypes> {
    type NumericTypes = NumericTypes;

    fn get_value(&self, identifier: &str) -> Option<&Value<Self::NumericTypes>> {
        self.variables.get(identifier)
    }

    fn call_function(
        &self,
        identifier: &str,
        argument: &Value<Self::NumericTypes>,
    ) -> EvalexprResultValue<Self::NumericTypes> {
        if let Some(function) = self.functions.get(identifier) {
            function.call(argument)
        } else {
            Err(EvalexprError::FunctionIdentifierNotFound(
                identifier.to_string(),
            ))
        }
    }

    fn are_builtin_functions_disabled(&self) -> bool {
        self.without_builtin_functions
    }

    fn set_builtin_functions_disabled(
        &mut self,
        disabled: bool,
    ) -> EvalexprResult<(), NumericTypes> {
        self.without_builtin_functions = disabled;
        Ok(())
    }
}

impl<NumericTypes: EvalexprNumericTypes> ContextWithMutableVariables
    for HashMapContext<NumericTypes>
{
    fn set_value(
        &mut self,
        identifier: String,
        value: Value<Self::NumericTypes>,
    ) -> EvalexprResult<(), NumericTypes> {
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
    fn remove_value(
        &mut self,
        identifier: &str
    ) -> EvalexprResult<Option<Value<Self::NumericTypes>>, Self::NumericTypes> {
        // Removes a value from the `self.variables`, returning the value at the key if the key was previously in the map.
        Ok(self.variables.remove(identifier))
    }
}

impl<NumericTypes: EvalexprNumericTypes> ContextWithMutableFunctions
    for HashMapContext<NumericTypes>
{
    fn set_function(
        &mut self,
        identifier: String,
        function: Function<NumericTypes>,
    ) -> EvalexprResult<(), Self::NumericTypes> {
        self.functions.insert(identifier, function);
        Ok(())
    }
}

impl<NumericTypes: EvalexprNumericTypes> IterateVariablesContext for HashMapContext<NumericTypes> {
    type VariableIterator<'a>
        = std::iter::Map<
        std::collections::hash_map::Iter<'a, String, Value<NumericTypes>>,
        fn((&String, &Value<NumericTypes>)) -> (String, Value<NumericTypes>),
    >
    where
        Self: 'a;
    type VariableNameIterator<'a>
        = std::iter::Cloned<std::collections::hash_map::Keys<'a, String, Value<NumericTypes>>>
    where
        Self: 'a;

    fn iter_variables(&self) -> Self::VariableIterator<'_> {
        self.variables
            .iter()
            .map(|(string, value)| (string.clone(), value.clone()))
    }

    fn iter_variable_names(&self) -> Self::VariableNameIterator<'_> {
        self.variables.keys().cloned()
    }
}

impl<NumericTypes: EvalexprNumericTypes> Default for HashMapContext<NumericTypes> {
    fn default() -> Self {
        Self {
            variables: Default::default(),
            functions: Default::default(),
            without_builtin_functions: false,
        }
    }
}

/// This macro provides a convenient syntax for creating a static context.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let ctx: HashMapContext<DefaultNumericTypes> = context_map! {
///     "x" => int 8,
///     "f" => Function::new(|_| Ok(Value::from_int(42)))
/// }.unwrap(); // Do proper error handling here
///
/// assert_eq!(eval_with_context("x + f()", &ctx), Ok(Value::from_int(50)));
/// ```
#[macro_export]
macro_rules! context_map {
    // Termination (allow missing comma at the end of the argument list)
    ( ($ctx:expr) $k:expr => Function::new($($v:tt)*) ) =>
        { $crate::context_map!(($ctx) $k => Function::new($($v)*),) };
    ( ($ctx:expr) $k:expr => int $v:expr ) =>
        { $crate::context_map!(($ctx) $k => int $v,)  };
    ( ($ctx:expr) $k:expr => float $v:expr ) =>
        { $crate::context_map!(($ctx) $k => float $v,)  };
    ( ($ctx:expr) $k:expr => $v:expr ) =>
        { $crate::context_map!(($ctx) $k => $v,)  };
    // Termination
    ( ($ctx:expr) ) => { Ok(()) };

    // The user has to specify a literal 'Function::new' in order to create a function
    ( ($ctx:expr) $k:expr => Function::new($($v:tt)*) , $($tt:tt)*) => {{
        $crate::ContextWithMutableFunctions::set_function($ctx, $k.into(), $crate::Function::new($($v)*))
            .and($crate::context_map!(($ctx) $($tt)*))
    }};
    // add an integer value, and chain the eventual error with the ones in the next values
    ( ($ctx:expr) $k:expr => int $v:expr , $($tt:tt)*) => {{
        $crate::ContextWithMutableVariables::set_value($ctx, $k.into(), $crate::Value::from_int($v.into()))
            .and($crate::context_map!(($ctx) $($tt)*))
    }};
    // add a float value, and chain the eventual error with the ones in the next values
    ( ($ctx:expr) $k:expr => float $v:expr , $($tt:tt)*) => {{
        $crate::ContextWithMutableVariables::set_value($ctx, $k.into(), $crate::Value::from_float($v.into()))
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
