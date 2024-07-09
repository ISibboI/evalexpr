use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::panic;
use std::panic::AssertUnwindSafe;
use thin_trait_object::thin_trait_object;
use crate::{BoxedOperatorRowTrait, Error, Value, ValueType};
use crate::Error::CustomError;


#[thin_trait_object]
pub trait CompiledTransposeCalculationTemplate : Send {
    fn schema(&self) -> HashMap<String,ValueType>;
    fn dependencies(&self) -> Vec<String>;
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error>;
    fn commit_row_wrapped(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error>{
        panic::catch_unwind(AssertUnwindSafe(|| self.commit_row(row, ordered_transpose_values, cycle_epoch))).map_err(|err| Error::CustomError(format!("{}", extract_message_from_any_error(err))))?
    }
}

pub fn extract_message_from_any_error(err: Box<dyn Any+ Send>) -> String {
    if let Some(s) = err.downcast_ref::<&str>() {
        s.to_owned()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.to_owned()
    } else {
        format!("Caught a panic with an unknown type.")
    }
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
pub fn context_result<C,T, E : Debug>(sself : Result<T, E>, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
{
    match sself {
        Ok(ok) => Ok(ok),
        Err(err) => Err(CustomError(format!("{} - {:?}", context, err))),
    }
}