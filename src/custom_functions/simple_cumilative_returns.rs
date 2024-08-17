﻿use crate::{Error, Value};

pub fn simple_cumulative_returns(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    let mut cumulative_return = 1.0f64;
    for &col_index in columns {
        if let Some(val) = row.get(col_index) {
            match val {
                Value::Float(val) => {
                    cumulative_return *= 1.0 + val;
                }
                Value::Int(val) => {
                    cumulative_return *= 1.0 + (*val as f64);
                }
                Value::Empty => {}
                _ => return Err(Error::NonNumericType),
            }
        }
    }

    Ok(Value::Float(cumulative_return - 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cumulative_returns_normal_operation() {
        let row = vec![Value::Float(0.1), Value::Float(0.2), Value::Float(0.3)];
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 1.2 * 1.3 - 1.0));
    }

    #[test]
    fn test_simple_cumulative_returns_partial_data() {
        let row = vec![Value::Float(0.1), Value::Empty, Value::Float(0.3)];
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 1.3 - 1.0));
    }

    #[test]
    fn test_simple_cumulative_returns_empty_input() {
        let row: Vec<Value> = vec![];
        let columns: Vec<usize> = vec![];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_simple_cumulative_returns_no_valid_columns() {
        let row = vec![Value::Empty, Value::Empty];
        let columns = vec![0, 1];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(0.0)); // Assuming no valid data means a return of 0.0
    }

    #[test]
    fn test_simple_cumulative_returns_mixed_data_types() {
        let row = vec![Value::Float(0.1), Value::Int(2), Value::Float(-0.1)];
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 3.0 * 0.9 - 1.0));
    }
}