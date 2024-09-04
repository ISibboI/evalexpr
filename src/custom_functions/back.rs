use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};

pub fn back(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }
    Ok(row.get_value_for_column(columns[0])?)
}

