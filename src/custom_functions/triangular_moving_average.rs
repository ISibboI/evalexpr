use crate::{Error, Value};

pub fn triangular_moving_average(row: &[Value], columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }
    let length = columns.len();
    let mut sum: Value = Value::Float(0f64); // Starting sum value
    let mut sum_weight: usize = 0;
    let half_length = (length / 2) + 1;
    for i in (0..=half_length).rev() {
        let weight = &half_length + 1;
        sum = (&sum + &row[columns[i]])?; // Add current value
        sum_weight += weight; // Update weight sum

        let mut k = half_length.clone();
        for j in 1..=half_length {
            if i + j < columns.len() {
                sum = (&sum +  &row[columns[i + j]])?; // Add value with weight
                sum_weight += k;
            }
            if i as isize - j as isize >= 0 {
                sum = (&sum + &row[columns[i - j]])?; // Add value with weight
                sum_weight += k;
            }
            k -= 1;
        }
    }
    sum / Value::Float(sum_weight as f64)
}


#[cfg(test)]
mod tests {
    use std::process::id;
    use super::*;

    #[test]
    fn test_triangular_moving_average_normal_operation() {

        let row = (0..110).map(|idx| Value::Float(idx as f64)).collect::<Vec<Value>>(); // Simple case with enough columns
        let columns = (0..110).collect::<Vec<usize>>(); // Simple case with enough columns
        let result = triangular_moving_average(&row, &columns);
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
