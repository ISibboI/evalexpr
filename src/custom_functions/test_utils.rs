use std::collections::HashMap;
use crate::{context, Error, EvalexprResult, OperatorRowTrait, Value};

pub struct MockRow {
    values: HashMap<String, Value>,
}

impl MockRow {
    pub fn new() -> Self {
        MockRow {
            values: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }
}

impl OperatorRowTrait for MockRow {
    // Implement required methods, simply accessing the `values` HashMap.
    // For simplicity, assuming methods to get and set values by column name.
    fn get_value(&self, column_name: &str) -> EvalexprResult<Value> {
        let option = self.values.get(column_name);
        match option {
            None => { Ok(Value::Empty)}
            Some(val) => {
                         Ok(val.clone())
            }
        }
    }

    fn set_value(&mut self, column_name: &str, value: Value) ->EvalexprResult<()> {
        self.values.insert(column_name.to_string(), value);
        Ok(())
    }

    fn set_row(&self, row: usize) {
        todo!()
    }

    fn call_function(&self, idt: &str, argument: &Value) -> EvalexprResult<Value> {
        todo!()
    }

    fn has_changes(&self) -> bool {
        todo!()
    }

}