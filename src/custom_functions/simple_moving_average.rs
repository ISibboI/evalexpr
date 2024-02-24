use crate::{Error, Value};

pub fn simple_moving_average(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }
    let mut sum: Value = Value::Float(0.0);
    let mut count = 0usize;

    for &col_index in columns {
        match row.get(col_index) {
            Some(Value::Float(val)) => {
                if let Value::Float(sum_val) = sum {
                    sum = Value::Float(sum_val + val);
                    count += 1;
                }
            }
            _ => continue,
        }
    }

    if count > 0 {
        if let Value::Float(sum_val) = sum {
            return Ok(Value::Float(sum_val / count as f64));
        }
    }

    Ok(Value::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average_normal_operation() {
        let row = vec![Value::Float(10.0), Value::Float(20.0), Value::Float(30.0), Value::Float(40.0)];
        let columns = vec![0, 1, 2, 3];
        let result = simple_moving_average(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(25.0));
    }

    #[test]
    fn test_simple_moving_average_partial_data() {
        let row = vec![Value::Float(10.0), Value::Empty, Value::Float(30.0), Value::Empty];
        let columns = vec![0, 1, 2, 3];
        let result = simple_moving_average(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(20.0));
    }

    #[test]
    fn test_simple_moving_average_empty_input() {
        let row: Vec<Value> = vec![];
        let columns: Vec<usize> = vec![];
        let result = simple_moving_average(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_simple_moving_average_no_valid_columns() {
        let row = vec![Value::Empty, Value::Empty];
        let columns = vec![0, 1];
        let result = simple_moving_average(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }
}
