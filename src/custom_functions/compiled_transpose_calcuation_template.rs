use std::collections::HashMap;
use std::fmt::Display;
use crate::{BoxedThinTraitContext, Error, Value, ValueType};
use crate::Error::CustomError;

pub trait CompiledTransposeCalculationTemplate {
    fn schema(&self) -> HashMap<String,ValueType>;
    fn dependencies(&self) -> Vec<String>;
    fn commit_row(self: &mut Box<Self>, context: &mut BoxedThinTraitContext, ordered_transpose_values: &[Value], current_position: usize) -> Result<(), Error>;
}

pub fn context<C,T>(sself : Option<T>, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
{
    match sself {
        Some(ok) => Ok(ok),
        None => Err(CustomError(format!("{}", context))),
    }
}