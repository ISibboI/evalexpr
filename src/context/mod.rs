//! A context defines methods to retrieve variable values and call functions for literals in an expression tree.
//! If mutable, it also allows to assign to variables.
//!
//! This crate implements two basic variants, the `EmptyContext`, that returns `None` for each identifier and cannot be manipulated, and the `HashMapContext`, that stores its mappings in hash maps.
//! The HashMapContext is type-safe and returns an error if the user tries to assign a value of a different type than before to an identifier.

use std::{borrow::Cow, collections::HashMap};
use indexmap::IndexMap;
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

    fn get_value_by_index(&self, identifier: &usize) -> Option<Cow<'_, Value>> {
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

#[derive(Clone, Debug, Default)]
pub struct IndexMapContext {
    variables: IndexMap<String, Value>,
    functions: HashMap<String, Function>,
}

impl HashMapContext {
    /// Constructs a `HashMapContext` with no mappings.
    pub fn new() -> Self {
        Default::default()
    }
}
impl IndexMapContext {
    /// Constructs a `HashMapContext` with no mappings.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Context for HashMapContext {
    fn get_value(&self, identifier: &str) -> Option<Cow<'_, Value>> {
        self.variables.get(identifier).map(Cow::Borrowed)
    }

    fn get_value_by_index(&self, identifier: &usize) -> Option<Cow<'_, Value>> {
        todo!("Get value by index not implemented for hashmap context")
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

impl Context for IndexMapContext {
    fn get_value(&self, identifier: &str) -> Option<Cow<'_, Value>> {
        self.variables.get(identifier).map(Cow::Borrowed)
    }

    fn get_value_by_index(&self, identifier: &usize) -> Option<Cow<'_, Value>> {
        self.variables.get_index(*identifier).map(|(_, v)| Cow::Borrowed(v))
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

impl ContextWithMutableVariables for IndexMapContext {
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
    fn get_value_by_index(&self, identifier: &usize) -> Option<Cow<'_, Value>>;
/// Retrieves the value of the given identifier.
    fn call_function(&self, idt: &str, argument: &Value) -> EvalexprResult<Value>;
}


#[cfg_attr(feature = "serde_json_support", thin_trait_object(generate_dotnet_wrappers=true))]
#[cfg_attr(not(feature = "serde_json_support"), thin_trait_object(generate_dotnet_wrappers=false))]
pub trait OperatorRowTrait {
    fn get_value(&self, identifier: &str) -> Result<Value,crate::Error>;
    fn get_value_by_index(&self, idx: &usize) -> Result<Value,crate::Error>;
    fn set_value(&mut self, identifier: &str, value: Value) -> Result<(),crate::Error>;
    fn get_value_for_column(&self, col: usize) -> Result<Value,crate::Error>;
    fn set_value_for_column(&mut self, col: usize, value: Value) -> Result<(),crate::Error>;
    fn set_row(&mut self, row: usize);
    fn call_function(&self, idt: &str, argument: Value) -> Result<Value, crate::Error>;
    fn has_changes(&self) -> Result<bool,crate::Error>;
    fn get_dirty_flags(&self) -> Result<Vec<usize>,crate::Error>;
}

#[cfg_attr(feature = "serde_json_support", thin_trait_object(generate_dotnet_wrappers=true))]
#[cfg_attr(not(feature = "serde_json_support"), thin_trait_object(generate_dotnet_wrappers=false))]
pub trait ActiveRowTrackerTrait {
    fn all_active_rows(&self) -> Result<Vec<usize>, crate::Error>;
    fn all_changes(&self) -> Result<Vec<(u8,usize)>, crate::Error>;
    fn handle_add(&mut self, row: usize) -> Result<(),crate::Error>;
    fn handle_update(&mut self, row: usize) -> Result<(),crate::Error>;
    fn handle_remove(&mut self, row: usize) -> Result<(),crate::Error>;
    fn is_active(&self, row: usize) -> Result<bool, crate::Error>;
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FFIColumn {
    pub name: String,
    pub data_type: ValueType,
    #[serde(default = "default_is_pk")]
    pub is_pk: bool,
    #[serde(default = "default_meta_data")]
    pub meta_data: String,
}

// Provide default values for the fields
fn default_is_pk() -> bool {
    false
}

fn default_meta_data() -> String {
    String::from("")
}

#[cfg_attr(feature = "serde_json_support", thin_trait_object(generate_dotnet_wrappers=true))]
#[cfg_attr(not(feature = "serde_json_support"), thin_trait_object(generate_dotnet_wrappers=false))]
pub trait OperatorSchemaTrait {
    fn get_schema(&self) -> Result<Vec<FFIColumn>, crate::Error>;
    fn get_column_for_index(&self, column: usize) -> Result<FFIColumn, crate::Error>;
    fn get_index_for_column(&self, column: String) -> Result<usize, crate::Error>;
    fn add_column(&mut self, column: FFIColumn) -> Result<(), crate::Error>;
    fn remove_column(&mut self, column_name: String) -> Result<(), crate::Error>;

    fn get_value(&self, identifier: String, row: usize) -> Result<Value,crate::Error>;
    fn set_value(&mut self, identifier: String, row: usize, value: Value) -> Result<(),crate::Error>;
    fn get_value_for_column(&self, col: usize, row: usize) -> Result<Value,crate::Error>;
    fn set_value_for_column(&mut self, col: usize, value: Value, row: usize) -> Result<(),crate::Error>;
}
#[cfg_attr(feature = "serde_json_support", thin_trait_object(generate_dotnet_wrappers=true))]
#[cfg_attr(not(feature = "serde_json_support"), thin_trait_object(generate_dotnet_wrappers=false))]
pub trait OperatorStatusContainerTrait {
    fn statuses(&self) -> Result<Vec<u8>, crate::Error>;
    fn changes(&self) -> Result<Vec<(u8, u8)>, crate::Error>;
    fn add(&mut self, status: u8, context: String) -> Result<bool, crate::Error>;
    fn remove(&mut self, status: u8) -> Result<(), crate::Error>;
    fn contains(&mut self, status: u8) -> Result<bool, crate::Error>;
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
