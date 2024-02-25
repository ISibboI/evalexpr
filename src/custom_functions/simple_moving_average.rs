use crate::{Error, Value};

pub fn simple_moving_average(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    let (sum, count) = columns.iter()
        .filter_map(|&col_index| match row.get(col_index) {
            Some(Value::Float(val)) => Some(*val),
            Some(Value::Int(val)) => Some(*val as f64),
            _ => None,
        })
        .fold((0.0f64, 0usize), |(acc_sum, acc_count), val| (acc_sum + val, acc_count + 1));

    if count > 0 {
        Ok(Value::Float(sum / count as f64))
    } else {
        Ok(Value::Empty)
    }
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

    //Time: 4.481µs
    #[test]
    fn test_triangular_moving_average_normal_operation() {

        let row = (0..1111111).map(|idx| Value::Float(idx as f64)).collect::<Vec<Value>>(); // Simple case with enough columns
        let columns = (0..110).collect::<Vec<usize>>(); // Simple case with enough columns
        let start = std::time::Instant::now();
        let result = simple_moving_average(&row, &columns);
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Float(avg) => {
                // Perform your assertion here based on expected calculation
                // This is just an example; the exact value will depend on your calculation
                assert!(avg > 0.0);
            },
            _ => panic!("Expected Value::Float from TMA calculation"),
        }
        println!("Time: {:?}", start.elapsed());
    }


}
