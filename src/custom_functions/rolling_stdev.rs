use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};

pub fn rolling_stdev(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    // Calculate the mean (average)
    let (sum, count) = columns.iter()
        .filter_map(|&col_index| match row.get_value_for_column(col_index).ok() {
            Some(Value::Float(val)) => Some(val),
            Some(Value::Int(val)) => Some(val as f64),
            _ => None,
        })
        .fold((0.0f64, 0usize), |(acc_sum, acc_count), val| (acc_sum + val, acc_count + 1));

    if count == 0 {
        return Ok(Value::Empty);
    }

    let mean = sum / count as f64;

    // Calculate the sum of squared deviations from the mean
    let sum_of_squared_diffs = columns.iter()
        .filter_map(|&col_index| match row.get_value_for_column(col_index).ok() {
            Some(Value::Float(val)) => Some(val),
            Some(Value::Int(val)) => Some(val as f64),
            _ => None,
        })
        .fold(0.0f64, |acc, val| acc + (val - mean).powi(2));

    // Calculate the standard deviation
    let variance = sum_of_squared_diffs / count as f64;
    let stdev = variance.sqrt();

    Ok(Value::Float(stdev))
}

#[cfg(test)]
mod tests {
    use crate::templates::test_utils::MockRow;
    use super::*;

    #[test]
    fn test_rolling_stdev_basic_case() {
        let row = MockRow::from_values(vec![
            Value::Float(10.0),
            Value::Float(12.0),
            Value::Float(23.0),
            Value::Float(23.0),
            Value::Float(16.0),
            Value::Float(23.0),
            Value::Float(21.0),
            Value::Float(16.0),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let result = rolling_stdev(&row, &columns).unwrap();

        if let Value::Float(stdev) = result {
            assert!((stdev - 4.899).abs() < 1e-3, "Expected stdev to be approximately 4.899, got {}", stdev);
        } else {
            panic!("Expected a Float value");
        }
    }

    #[test]
    fn test_rolling_stdev_mixed_values() {
        let row = MockRow::from_values(vec![
            Value::Float(1.0),
            Value::Int(3),
            Value::Float(4.0),
            Value::Int(2),
            Value::Float(5.0),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = rolling_stdev(&row, &columns).unwrap();

        if let Value::Float(stdev) = result {
            assert!((stdev - 1.414).abs() < 1e-3, "Expected stdev to be approximately 1.414, got {}", stdev);
        } else {
            panic!("Expected a Float value");
        }
    }

    #[test]
    fn test_rolling_stdev_with_empty_values() {
        let row = MockRow::from_values(vec![
            Value::Float(0.1),
            Value::Empty,
            Value::Float(0.3),
            Value::Empty,
            Value::Float(0.5),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = rolling_stdev(&row, &columns).unwrap();

        if let Value::Float(stdev) = result {
            assert!((stdev - 0.163).abs() < 1e-3, "Expected stdev to be approximately 0.163, got {}", stdev);
        } else {
            panic!("Expected a Float value");
        }
    }

    #[test]
    fn test_rolling_stdev_empty_columns() {
        let row = MockRow::from_values(vec![
            Value::Float(1.0),
            Value::Int(3),
            Value::Float(4.0),
        ]).into_boxed();
        let columns: Vec<usize> = vec![];
        let result = rolling_stdev(&row, &columns).unwrap();

        assert_eq!(result, Value::Empty, "Expected an Empty value when no columns are provided");
    }

    #[test]
    fn test_rolling_stdev_single_value() {
        let row = MockRow::from_values(vec![
            Value::Float(2.5),
        ]).into_boxed();
        let columns = vec![0];
        let result = rolling_stdev(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(0.0), "Expected stdev to be 0.0 when only one value is provided");
    }

    #[test]
    fn test_rolling_stdev_all_empty_values() {
        let row = MockRow::from_values(vec![
            Value::Empty,
            Value::Empty,
            Value::Empty,
        ]).into_boxed();
        let columns = vec![0, 1, 2];
        let result = rolling_stdev(&row, &columns).unwrap();

        assert_eq!(result, Value::Empty, "Expected an Empty value when all columns contain Empty values");
    }
}

