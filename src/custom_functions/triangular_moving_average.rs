use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};
use crate::Error::UnsupportedOperation;

pub fn columns_len(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    Ok(Value::Int(columns.len() as i64))
}
pub fn triangular_moving_average(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    if columns.len() < 3 {
        return Ok(Value::Empty); // Not engh data to calculate
    }

    let mut total_sum = 0.0;
    let mut total_weight = 0;
    let half_length = columns.len() / 2;

    for i in 0..columns.len() {
        let mut sum: f64 = 0.0;
        let mut sum_weight = 0;
        let mut k = 1;

        // Sum over the symmetric window around the current index `i`
        for j in 0..=half_length {
            // Forward direction
            if i + j < columns.len() {
                if let Some(price) = get_price(row, columns, i + j) {
                    let weight = if j == 0 { half_length + 1 } else { half_length + 1 - j };
                    sum += price * weight as f64;
                    sum_weight += weight;
                }
            }

            // Backward direction, skipping the center when j == 0
            if j != 0 && i >= j {
                if let Some(price) = get_price(row, columns, i - j) {
                    let weight = half_length + 1 - j;
                    sum += price * weight as f64;
                    sum_weight += weight;
                }
            }
        }

        total_sum += sum;
        total_weight += sum_weight;
    }

    if total_weight > 0 {
        Ok(Value::Float(total_sum / total_weight as f64))
    } else {
        Ok(Value::Empty)
    }
}

fn get_price(row: &BoxedOperatorRowTrait, columns: &[usize], index: usize) -> Option<f64> {
    row.get_value_for_column(columns[index]).ok().and_then(|value| match value {
        Value::Float(val) => Some(val),
        Value::Int(val) => Some(val as f64),
        _ => None,
    })
}


#[cfg(test)]
mod tests {
    use std::process::id;
    use crate::templates::test_utils::{MockIndex, MockIndexHolder, MockRow};
    use super::*;

    //Time: 4.481µs
    #[test]
    fn test_triangular_moving_average_normal_operation() {
        let mock_index = MockIndexHolder::new();
        let row = MockRow::from_values((0..1111111).map(|idx| Value::Float(idx as f64)).collect::<Vec<Value>>(), &mock_index); // Simple case with enough columns
        let columns = (0..5).collect::<Vec<usize>>(); // Simple case with enough columns
        let start = std::time::Instant::now();
        let result = triangular_moving_average(&BoxedOperatorRowTrait::new(row), &columns);
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Float(avg) => {
                // Perform your assertion here based on expected calculation
                // This is just an example; the exact value will depend on your calculation
                println!("Average: {}", avg);
                assert!(avg > 0.0);
            }
            _ => panic!("Expected Value::Float from TMA calculation"),
        }
        println!("Time: {:?}", start.elapsed());
    }


    #[test]
    fn test_triangular_moving_average_empty_input() {
        let mock_index = MockIndexHolder::new();
        let row = MockRow::from_values(vec![], &mock_index).into_boxed();
        let columns: Vec<usize> = vec![];
        let result = triangular_moving_average(&row, &columns);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Empty); // Assuming Value::Empty for empty input
    }


    #[test]
    fn test_triangular_moving_average_empty() {
        let mock_index = MockIndexHolder::new();
        let row = MockRow::from_values(vec![],&mock_index).into_boxed();
        let columns = vec![];
        let result = triangular_moving_average(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_triangular_moving_average_basic() {
        // Setup a simple scenario
        
        let mock_index_holder = MockIndexHolder::new();
        let row =  MockRow::from_values(vec![Value::Int(10), Value::Int(20), Value::Int(30), Value::Int(40), Value::Int(50)], &mock_index_holder).into_boxed();
        let columns = vec![0, 1, 2, 3, 4]; // Direct mapping for simplicity
        let result = triangular_moving_average(&row, &columns).unwrap();

        // Expected calculation goes here based on the specific logic of triangular_moving_average
        // For simplicity, let's say we expect the average of all values
        let expected_average = Value::Float(36.08695652173913); // Placeholder for the actual expected result

        assert_eq!(result, expected_average);
    }

    #[test]
    fn test_triangular_moving_average_with_floats() {
        // Test the function with floating point numbers
        let mock_index_holder = MockIndexHolder::new();
        let row = MockRow::from_values(vec![
            Value::Float(10.5),
            Value::Float(20.5),
            Value::Float(30.5),
            Value::Float(40.5),
            Value::Float(50.5)
        ], &mock_index_holder).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = triangular_moving_average(&row, &columns).unwrap();

        // Calculate expected result based on provided logic
        let expected_average = Value::Float(30.5); // Placeholder

        assert_eq!(result, expected_average);
    }

    #[test]
    fn test_triangular_moving_average_invalid_values() {
        // Test how the function handles invalid (non-numeric) values
        let mock_index_holder = MockIndexHolder::new();
        let row =  MockRow::from_values(vec![Value::Empty, Value::Int(20), Value::Empty, Value::Int(40), Value::Empty], &mock_index_holder).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = triangular_moving_average(&row, &columns).unwrap();

        // Expected result considering how non-numeric values are handled
        let expected_average = Value::Float(30.0); // Placeholder, assuming non-numeric values are ignored

        assert_eq!(result, expected_average);
    }

    // Add more tests as needed, especially to cover the error cases your implementation might have
}
