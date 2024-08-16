use crate::{Error, Value};

pub fn back(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Float(100f64));
    }
    Ok(row[columns[0]].clone())
}

