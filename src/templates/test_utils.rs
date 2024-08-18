use std::collections::HashMap;
use indexmap::IndexMap;
use crate::{context, BoxedOperatorRowTrait, Error, EvalexprResult, OperatorRowTrait, Value};

pub struct MockRow {
    values: IndexMap<String, Value>,
}

impl MockRow {
    pub fn new() -> Self {
        MockRow {
            values: IndexMap::new(),
        }
    }

    pub fn from_values(row: Vec<Value>) -> Self {
        let mut mock_row = MockRow::new();

        // Iterate over the values and insert them into the mock_row with a key
        for (idx, value) in row.into_iter().enumerate() {
            let key = format!("col_{}", idx); // Generate a key for each value
            mock_row.insert_value(key, value);
        }

        mock_row
    }
    
    pub fn into_boxed<'inner>(self) -> BoxedOperatorRowTrait<'inner> {
        BoxedOperatorRowTrait::new(self)
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

    fn get_value_for_column(&self, col: usize) -> Result<Value, Error> {
        let option = self.values.get_index(col);
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