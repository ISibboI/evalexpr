use std::collections::HashMap;
use indexmap::IndexMap;
use crate::{context, generate_column_name, BoxedOperatorRowTrait, Error, EvalexprResult, OperatorRowTrait, Value};
use crate::context::{BoxedTransposeColumnIndex, TransposeColumnIndex, TransposeColumnIndexHolder};

#[derive(Clone)]
pub struct MockRow<'a> {
    values: HashMap<usize, Value>,
    mock_index: &'a MockIndexHolder,
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
    pub fn register_index(&mut self, index_name: String, transpose_values: &[Value]) {
        let index = MockIndex::from_transpose_values(index_name.clone(), transpose_values, self.offset);
        if  self.values.contains_key(&index_name) {
            panic!("index {} already exists", index_name);
        }
        self.values.insert(index_name.clone(), index);
        for (key, idx) in &self.values.get(&index_name).unwrap().values {
            if self.output_column_indexes.contains_key(key) {
                panic!("Column {} already exists",key);
            }
            self.output_column_indexes.insert(key.clone(), *idx);
        }
        self.offset += transpose_values.len();
    }

    pub fn get_index_for_column_full(&self, column_name: String) -> Result<usize, Error> {
        let option = self.output_column_indexes.get(&column_name);
        match option {
            None => { Err(Error::CustomError(format!("Column name {column_name} not found"))) }
            Some(val) => {
                Ok(*val)
            }
        }
    }
}

impl<'a> TransposeColumnIndexHolder for &'a MockIndexHolder {
    fn get_index_for_column(&self, column_name: String) -> Result<BoxedTransposeColumnIndex<'static>, Error> {
        let option = self.values.get(&column_name);
        match option {
            None => { Err(Error::CustomError(format!("Column name {column_name} not found"))) }
            Some(val) => {
                let raw = BoxedTransposeColumnIndex::new(val).into_raw();
                let val = unsafe { BoxedTransposeColumnIndex::from_raw(raw) };
                Ok(val)
            }
        }
    }

    fn get_index_vec(&self, column_name: String) -> Result<Vec<usize>, Error> {
        let option = self.values.get(&column_name);
        match option {
            None => { Err(Error::CustomError(format!("Column name {column_name} not found"))) }
            Some(val) => {
                let vec = val.values.values().cloned().collect();
                Ok(vec)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockIndex {
    values: IndexMap<String, usize>,
}

impl MockIndex {
    pub fn from_transpose_values(index_name: String, transpose_values: &[Value], offset: usize) -> Self {
        let mut mock_index = MockIndex {
            values: IndexMap::new(),
        };
        for (idx, value) in transpose_values.into_iter().enumerate() {
            let key = generate_column_name(&index_name, value); // Generate a key for each value
            let i    = offset + idx;
            mock_index.values.insert(key, i);
        }
        mock_index
    }
}

impl TransposeColumnIndex for &MockIndex {
    fn col_idx(&self, transpose_index: usize) -> Result<usize, Error> {
        let option = self.values.get_index(transpose_index);
        match option {
            None => { Err(Error::CustomError("Transpose index not found".to_string())) }
            Some((_nm, val)) => {
                Ok(*val)
            }
        }
    }
}

impl<'a> MockRow<'a> {
    pub fn new(mock_index: &'a MockIndexHolder) -> Self {
        MockRow {
            values: HashMap::new(),
            mock_index,
        }
    }

    pub fn from_values(row: Vec<Value>, mock_index: &'a MockIndexHolder) -> Self {
        let mut mock_row = MockRow::new(mock_index);

        // Iterate over the values and insert them into the mock_row with a key
        for (idx, value) in row.into_iter().enumerate() {
            let key = format!("col_{}", idx); // Generate a key for each value
            mock_row.insert_value(key, value);
        }

        mock_row
    }


    pub fn set_value_for_transpose_index(&mut self, column_name: &str, transpose_idx: usize, value: Value) -> Result<(), crate::Error> {
        let output_column_index = self.mock_index.get_index_for_column(column_name.to_owned())?.col_idx(transpose_idx)?;
        self.set_value_for_column(output_column_index, value)?;
        Ok(())
    }

    pub fn get_value_for_transpose_index(&self, column_name: &str, transpose_idx: usize) -> Result<Value, Error> {
        let output_column_index = self.mock_index.get_index_for_column(column_name.to_owned())?.col_idx(transpose_idx)?;
        self.get_value_for_column(output_column_index)
    }

    pub fn into_boxed(self) -> BoxedOperatorRowTrait<'a> {
        BoxedOperatorRowTrait::new(self)
    }
    pub fn insert_value(&mut self, key: String, value: Value) -> Result<(), Error> {
        let index = self.mock_index.get_index_for_column_full(key.clone())?;
        self.values.insert(index, value);
        Ok(())
    }
}

impl<'a> OperatorRowTrait for MockRow<'a> {
    // Implement required methods, simply accessing the `values` HashMap.
    // For simplicity, assuming methods to get and set values by column name.
    fn get_value(&self, column_name: &str) -> Result<Value, crate::Error> {
        let index_for_column = self.mock_index.get_index_for_column_full(column_name.to_string())?;
        let option = self.values.get(&index_for_column);
        match option {
            None => { Ok(Value::Empty) }
            Some(val) => {
                Ok(val.clone())
            }
        }
    }

    fn get_value_for_column(&self, col: usize) -> Result<Value, Error> {
        let option = self.values.get(&col);
        match option {
            None => { Ok(Value::Empty) }
            Some(val) => {
                Ok(val.clone())
            }
        }
    }

    fn set_value(&mut self, column_name: &str, value: Value) -> Result<(), crate::Error> {
        let index_for_column = self.mock_index.get_index_for_column_full(column_name.to_string())?;
        self.values.insert(index_for_column, value);
        Ok(())
    }


    fn set_value_for_column(&mut self, col: usize, value: Value) -> Result<(), crate::Error> {
        self.values.insert(col, value);
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


    fn get_dirty_flags(&self) -> Result<Vec<usize>, crate::Error> {
        todo!()
    }

    fn get_values(&self) -> Result<Vec<Value>, Error> {
        let mut result = vec![Value::Empty;self.mock_index.output_column_indexes.len()];
        for (nm, val) in &self.mock_index.output_column_indexes {
            result[*val] = self.values.get(val).cloned().unwrap_or(Value::Empty);
        }
        Ok(result)
    }

    fn set_values_for_columns(&mut self, columns: Vec<usize>, mut values: Vec<Value>) -> Result<(), Error> {
        for column in columns {
            self.set_value_for_column(column.clone(), values.get(column).unwrap().clone())?;
        }
        Ok(())
    }

    fn get_values_for_columns(&self, columns: Vec<usize>) -> Result<Vec<Value>, Error> {
        let mut result = vec![Value::Empty;self.mock_index.output_column_indexes.len()];
        println!("Getting values for columns {:?}", columns);
        for column in columns {
            let value = self.get_value_for_column(column)?;
            println!("Value for column {} is {:?}", column, value);
            result[column] = value;
        }
        Ok(result)
    }
}


impl<'a> OperatorRowTrait for &mut MockRow<'a> {
    // Implement required methods, simply accessing the `values` HashMap.
    // For simplicity, assuming methods to get and set values by column name.
    fn get_value(&self, column_name: &str) -> Result<Value, crate::Error> {
        let index_for_column = self.mock_index.get_index_for_column_full(column_name.to_string())?;
        let option = self.values.get(&index_for_column);
        match option {
            None => { Ok(Value::Empty) }
            Some(val) => {
                Ok(val.clone())
            }
        }
    }

    fn get_value_for_column(&self, col: usize) -> Result<Value, Error> {
        let option = self.values.get(&col);
        match option {
            None => { Ok(Value::Empty) }
            Some(val) => {
                Ok(val.clone())
            }
        }
    }

    fn set_value(&mut self, column_name: &str, value: Value) -> Result<(), crate::Error> {
        println!("Setting value for column {} to {:?}", column_name, value);
        let index_for_column = self.mock_index.get_index_for_column_full(column_name.to_string())?;
        self.values.insert(index_for_column, value);
        Ok(())
    }

    fn get_values_for_columns(&self, columns: Vec<usize>) -> Result<Vec<Value>, Error> {
        let mut result = vec![Value::Empty;self.mock_index.output_column_indexes.len()];
        println!("Getting values for columns {:?}", columns);
        for column in columns {
            let value = self.get_value_for_column(column)?;
            println!("Value for column {} is {:?}", column, value);
            result[column] = value;
        }
        Ok(result)
    }


    fn set_value_for_column(&mut self, col: usize, value: Value) -> Result<(), crate::Error> {
        let (column_name, idx) = self.mock_index.output_column_indexes.get_index(col).ok_or_else(|| Error::CustomError(format!("Column not found {}", col)))?;
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


    fn get_dirty_flags(&self) -> Result<Vec<usize>, crate::Error> {
        todo!()
    }

    fn get_values(&self) -> Result<Vec<Value>, Error> {
        let mut result = vec![Value::Empty;self.mock_index.output_column_indexes.len()];
        for (nm, val) in &self.mock_index.output_column_indexes {
            result[*val] = self.values.get(val).cloned().unwrap_or(Value::Empty);
        }
        Ok(result)
    }

    fn set_values_for_columns(&mut self, columns: Vec<usize>, mut values: Vec<Value>) -> Result<(), Error> {
        for column in columns {
            self.set_value_for_column(column.clone(), values.get(column).unwrap().clone())?;
        }
        Ok(())
    }
}