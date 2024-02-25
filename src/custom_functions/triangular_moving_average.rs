use crate::{Error, Value};
use crate::Error::UnsupportedOperation;

pub fn triangular_moving_average(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    let mut sum: f64 = 0.0; // Use primitive type for intermediate summation
    let mut sum_weight: usize = 0;
    let length = columns.len();
    let half_length = (length / 2) + 1;
    macro_rules! process_value {
        ($val:expr, $weight:expr, $sum:expr, $sum_weight:expr) => {
            match $val {
                Value::Float(val) => {
                    *$sum += *val as f64 * $weight as f64;
                },
                Value::Int(val) => {
                    *$sum += *val as f64 * $weight as f64;
                },

                _ => {} // Optionally handle other variants
            }
            *$sum_weight += $weight;

        };
    }
    for i in 0..=half_length {
        let weight = half_length + 1 - i; // Adjust weight based on distance from center

        if let Some(val) = row.get(columns[i]) {
            process_value!(val, weight, &mut sum, &mut sum_weight);
        }

        if i != 0 && i < half_length { // Check to avoid double-counting the middle for odd lengths
            if let Some(sym_val) = row.get(columns[length - 1 - i]) {
                process_value!(sym_val, weight, &mut sum, &mut sum_weight);
            }
        }
    }

    if sum_weight > 0 {
        Ok(Value::Float(sum / sum_weight as f64))
    } else {
        Ok(Value::Empty) // Handle case where no valid data was added
    }
}


#[cfg(test)]
mod tests {
    use std::process::id;
    use super::*;

    //Time: 4.481µs
    #[test]
    fn test_triangular_moving_average_normal_operation() {
        let row = (0..1111111).map(|idx| Value::Float(idx as f64)).collect::<Vec<Value>>(); // Simple case with enough columns
        let columns = (0..110).collect::<Vec<usize>>(); // Simple case with enough columns
        let start = std::time::Instant::now();
        let result = triangular_moving_average(&row, &columns);
        assert!(result.is_ok());
        let value = result.unwrap();
        match value {
            Value::Float(avg) => {
                // Perform your assertion here based on expected calculation
                // This is just an example; the exact value will depend on your calculation
                assert!(avg > 0.0);
            }
            _ => panic!("Expected Value::Float from TMA calculation"),
        }
        println!("Time: {:?}", start.elapsed());
    }


    #[test]
    fn test_triangular_moving_average_empty_input() {
        let row: Vec<Value> = vec![];
        let columns: Vec<usize> = vec![];
        let result = triangular_moving_average(&row, &columns);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Empty); // Assuming Value::Empty for empty input
    }

    // Add more tests as needed, especially to cover the error cases your implementation might have
}
