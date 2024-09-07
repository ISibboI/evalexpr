use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};

pub  fn get_value_indirect<'a>(values: &'a Vec<Value>, column_index: &Vec<usize>, idx: usize) -> Result<&'a Value, Error> {
    let column = column_index.get(idx).ok_or_else(|| Error::CustomError(format!("Column not found in index{}", idx)))?;
    let result = values.get(*column).ok_or_else(|| Error::CustomError(format!("Column {column} not found in row")))?;
    Ok(result)
}

pub  fn get_value_indirect_from_row<'a>(row: &BoxedOperatorRowTrait, column_index: &Vec<usize>, idx: usize) -> Result<Value, Error> {
    let column = column_index.get(idx).ok_or_else(|| Error::CustomError(format!("Column not found in index{}", idx)))?;
    row.get_value_for_column(*column)
}

pub  fn set_value_indirect<'a>(values: &'a mut  Vec<Value>,dirty_columns: &'a mut  Vec<usize>, column_index: &Vec<usize>, idx: usize, value: Value) -> Result<(), Error> {
    let column = column_index.get(idx).ok_or_else(|| Error::CustomError(format!("Column not found in index{}", idx)))?;
    let mut result = values.get_mut(*column).ok_or_else(|| Error::CustomError(format!("Column {column} not found in row")))?;
    *result = value.into_owned();
    dirty_columns.push(*column);
    Ok(())
}pub  fn set_value_indirect_if_some<'a>(values: &'a mut  Vec<Value>,dirty_columns: &'a mut  Vec<usize>, column_index: &Vec<usize>, idx: usize, value: Option<Value>) -> Result<(), Error> {
    if let Some(value) = value {
        let column = column_index.get(idx).ok_or_else(|| Error::CustomError(format!("Column not found in index{}", idx)))?;
        let mut result = values.get_mut(*column).ok_or_else(|| Error::CustomError(format!("Column {column} not found in row")))?;
        *result = value.into_owned();
        dirty_columns.push(*column);
    }
    Ok(())
}