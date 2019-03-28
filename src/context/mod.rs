use std::collections::HashMap;

use EvalexprError;
use EvalexprResult;
use function::Function;
use value::value_type::ValueType;

use crate::value::Value;

/// A mutable context for an expression tree.
///
/// A context defines methods to retrieve values and functions for literals in an expression tree.
/// In addition, it also allows the manipulation of values and functions.
/// This crate implements two basic variants, the `EmptyContext`, that returns `None` for each identifier and cannot be manipulated, and the `HashMapContext`, that stores its mappings in hash maps.
/// The HashMapContext is type-safe and returns an error if the user tries to assign a value of a different type than before to an identifier.
pub trait Context {
    /// Returns the value that is linked to the given identifier.
    fn get_value(&self, identifier: &str) -> Option<&Value>;

    /// Returns the function that is linked to the given identifier.
    fn get_function(&self, identifier: &str) -> Option<&Function>;

    /// Links the given value to the given identifier.
    fn set_value(&mut self, _identifier: String, _value: Value) -> EvalexprResult<()> {
        Err(EvalexprError::ContextNotManipulable)
    }

    /// Links the given function to the given identifier.
    fn set_function(&mut self, _identifier: String, _function: Function) -> EvalexprResult<()> {
        Err(EvalexprError::ContextNotManipulable)
    }
}

/// A context that returns `None` for each identifier.
pub struct EmptyContext;

impl Context for EmptyContext {
    fn get_value(&self, _identifier: &str) -> Option<&Value> {
        None
    }

    fn get_function(&self, _identifier: &str) -> Option<&Function> {
        None
    }
}

/// A context that stores its mappings in hash maps.
///
/// *Value and function mappings are stored independently, meaning that there can be a function and a value with the same identifier.*
///
/// This context is type-safe, meaning that an identifier that is assigned a value of some type once cannot be assigned a value of another type.
#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct HashMapContext {
    variables: HashMap<String, Value>,
    #[cfg_attr(feature = "serde_support", serde(skip))]
    functions: HashMap<String, Function>,
}

impl HashMapContext {
    /// Constructs a `HashMapContext` with no mappings.
    pub fn new() -> Self {
        Self {
            variables: Default::default(),
            functions: Default::default(),
        }
    }
}

impl Context for HashMapContext {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.variables.get(identifier)
    }

    fn get_function(&self, identifier: &str) -> Option<&Function> {
        self.functions.get(identifier)
    }

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

    fn set_function(&mut self, identifier: String, function: Function) -> EvalexprResult<()> {
        self.functions.insert(identifier.into(), function);
        Ok(())
    }
}
