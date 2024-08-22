use std::collections::HashMap;
use indexmap::IndexMap;
use crate::{context, generate_column_name, BoxedOperatorRowTrait, Error, EvalexprResult, OperatorRowTrait, Value};
use crate::context::{BoxedTransposeColumnIndex, TransposeColumnIndex, TransposeColumnIndexHolder};

#[derive(Clone)]
pub struct MockRow<'a> {
    values: IndexMap<String, Value>,
    mock_index: &'a MockIndexHolder
}

pub struct MockIndexHolder {
    offset: usize,
    values: IndexMap<String, MockIndex>,
    output_column_indexes: IndexMap<String, usize>,
}

impl MockIndexHolder {
    pub fn new() -> Self {
        MockIndexHolder {
            offset: 0,
            values: IndexMap::new(),
            output_column_indexes: Default::default(),
        }
    }
    pub  fn register_index(&mut self, index_name: String, transpose_values: &[Value]) {
        let index = MockIndex::from_transpose_values(index_name.clone(), transpose_values, self.offset);
        self.values.insert(index_name.clone(), index);
        for (key,idx) in &self.values.get(&index_name).unwrap().values {
            self.output_column_indexes.insert(key.clone(),*idx);
        }
        self.offset += transpose_values.len();
    }
}

impl<'a> TransposeColumnIndexHolder for &'a MockIndexHolder{
    fn get_index_for_column(&self, column_name: String) -> Result<BoxedTransposeColumnIndex<'static>, Error> {
        let option = self.values.get(&column_name);
        match option {
            None => { Err(Error::CustomError("Column name not found".to_string()))}
            Some(val) => {
                let raw =BoxedTransposeColumnIndex::new(val).into_raw(); 
                let val = unsafe{BoxedTransposeColumnIndex::from_raw(raw)};
                Ok(val)
            }
        }
    }

    fn get_index_vec(&self, column_name: String) -> Result<Vec<usize>, Error> {
        let option = self.values.get(&column_name);
        match option {
            None => { Err(Error::CustomError("Column name not found".to_string()))}
            Some(val) => {
                Ok(val.values.values().cloned().collect())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockIndex {
    values: IndexMap<String, usize>,
}

impl MockIndex {
    pub  fn from_transpose_values(index_name: String, transpose_values: &[Value], offset: usize) -> Self {
        let mut mock_index = MockIndex {
            values: IndexMap::new(),
        };
        for (idx,value) in transpose_values.into_iter().enumerate() {
            let key = generate_column_name(&index_name,value); // Generate a key for each value
            mock_index.values.insert(key, offset + idx);
        }
        mock_index
    }
}

impl TransposeColumnIndex for &MockIndex{
    fn col_idx(&self, transpose_index: usize) -> Result<usize, Error> {
        let option = self.values.get_index(transpose_index);
        match option {
            None => { Err(Error::CustomError("Transpose index not found".to_string()))}
            Some((_nm,val)) => {
                Ok(*val)
            }
        }
    }
}

impl<'a> MockRow<'a>{
    pub fn new(mock_index: &'a MockIndexHolder) -> Self {
        MockRow {
            values: IndexMap::new(),
            mock_index
        }
    }

    pub fn from_values(row: Vec<Value>,mock_index: &'a MockIndexHolder) -> Self {
        let mut mock_row = MockRow::new(mock_index);

        // Iterate over the values and insert them into the mock_row with a key
        for (idx, value) in row.into_iter().enumerate() {
            let key = format!("col_{}", idx); // Generate a key for each value
            mock_row.insert_value(key, value);
        }

        mock_row
    }
    
    pub fn into_boxed(self) -> BoxedOperatorRowTrait<'a> {
        BoxedOperatorRowTrait::new(self)
    }
    pub fn insert_value(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }
}

impl<'a> OperatorRowTrait for MockRow<'a> {
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
        println!("Setting value for column {} to {:?}",column_name,value);
        self.values.insert(column_name.to_string(), value);
        Ok(())
    }


    fn set_value_for_column(&mut self, col: usize, value: Value) -> Result<(),crate::Error> {
        let (column_name,idx) = self.mock_index.output_column_indexes.get_index(col).ok_or_else(||Error::CustomError(format!("Column not found {}",col)))?;
        self.set_value(column_name, value.clone())?;
        Ok(())
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

    fn get_values(&self) -> Result<Vec<Value>, Error> {
        let mut result = vec![];
        for (nm,val) in  &self.values{
            result.push(val.clone())
        }
        for (idx,value) in &self.mock_index.values {
            for value in &value.values {
                result.push(Value::Empty)
            }
        }
        Ok(result)
    }

    fn set_values_for_columns(&mut self, columns: Vec<usize>, mut values: Vec<Value>) -> Result<(), Error> {
        for column in columns {
            self.set_value_for_column(column.clone(), values.remove(column))?;
        }
        Ok(())
    }
}