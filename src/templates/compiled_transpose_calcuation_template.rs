use std::collections::HashMap;
use std::fmt::Display;
use thin_trait_object::thin_trait_object;
use crate::{BoxedOperatorRowTrait, Error, Value, ValueType};
use crate::Error::CustomError;


#[thin_trait_object]
pub trait CompiledTransposeCalculationTemplate : Send {
    fn schema(&self) -> HashMap<String,ValueType>;
    fn dependencies(&self) -> Vec<String>;
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error>;
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