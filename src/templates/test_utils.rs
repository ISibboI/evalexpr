use std::collections::HashMap;
use indexmap::IndexMap;
use crate::{context, Error, EvalexprResult, OperatorRowTrait, Value};

pub struct MockRow {
    values: IndexMap<String, Value>,
}

impl MockRow {
    pub fn new() -> Self {
        MockRow {
            values: IndexMap::new(),
        }
    }

    pub fn insert_value(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }
}

impl OperatorRowTrait for MockRow {
    // Implement required methods, simply accessing the `values` HashMap.
    // For simplicity, assuming methods to get and set values by column name.
    fn get_value(&self, column_name: &str) -> Result<Value,crate::Error> {
        let option = self.values.get(column_name);
        match option {
            None => { Ok(Value::Empty)}
            Some(val) => {
                         Ok(val.clone())
            }
        }
    }

    fn get_value_by_index(&self, idx: usize) -> Result<Value, Error> {
        let option = self.values.get_index(idx);
        match option {
            None => { Ok(Value::Empty)}
            Some(val) => {
                Ok(val.1.clone())
            }
        }
    }

    fn set_value(&mut self, column_name: &str, value: Value) ->Result<(),crate::Error> {
        self.values.insert(column_name.to_string(), value);
        Ok(())
    }

    fn get_value_for_column(&self, col: usize) -> Result<Value,crate::Error> {
        todo!()
    }

    fn set_value_for_column(&mut self, col: usize, value: Value) -> Result<(),crate::Error> {
        todo!()
    }

    fn set_row(&mut self, row: usize) {
        todo!()
    }

    fn call_function(&self, idt: &str, argument: Value) -> Result<Value, Error> {
        todo!()
    }

    fn has_changes(&self) -> Result<bool, Error> {
        todo!()
    }


    fn get_dirty_flags(&self) -> Result<Vec<usize>,crate::Error> {
        todo!()
    }
}