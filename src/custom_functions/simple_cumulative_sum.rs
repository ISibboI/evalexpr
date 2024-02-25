use crate::{Error, Value};

pub fn simple_cumulative_sum(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }
    // Initialize sum directly as f64 to avoid repeated matching and unwrapping of Value::Float.
    let mut sum = 0.0f64;
    // Iterate through columns once, accumulating only if the value is a Float.
    for &col_index in columns {
        // Directly access the value without intermediate matching if it's safe (i.e., within bounds)
        // This avoids the need for matching on Option from row.get()
        if let Some(val) = row.get(col_index) {
            sum += val.as_number().map_err(|_| Error::NonNumericType)?;
        }
    }

    // Only wrap the final sum into Value::Float here.
    Ok(Value::Float(sum))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cumulative_sum_normal_operation() {
        let row = vec![Value::Float(10.0), Value::Float(20.0), Value::Float(30.0), Value::Float(40.0)];
        let columns = vec![0, 1, 2, 3];
        let result = simple_cumulative_sum(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(100.0));
    }

    #[test]
    fn test_simple_cumulative_sum_partial_data() {
        let row = vec![Value::Float(10.0), Value::Empty, Value::Float(30.0), Value::Empty];
        let columns = vec![0, 1, 2, 3];
        let result = simple_cumulative_sum(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(40.0));
    }

    #[test]
    fn test_simple_cumulative_sum_empty_input() {
        let row: Vec<Value> = vec![];
        let columns: Vec<usize> = vec![];
        let result = simple_cumulative_sum(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_simple_cumulative_sum_no_valid_columns() {
        let row = vec![Value::Empty, Value::Empty];
        let columns = vec![0, 1];
        let result = simple_cumulative_sum(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(0.0)); // Assuming Value::Float(0.0) for no valid data
    }
}
