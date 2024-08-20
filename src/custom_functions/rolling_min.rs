use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};

pub fn rolling_min(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    let mut min_value: Option<f64> = None;

    for &col_index in columns.iter() {
        if let Some(value) = row.get_value_for_column(col_index).ok() {
            match value {
                Value::Float(val) => {
                    min_value = Some(min_value.map_or(val, |min| min.min(val)));
                },
                Value::Int(val) => {
                    let val = val as f64;
                    min_value = Some(min_value.map_or(val, |min| min.min(val)));
                },
                _ => {}
            }
        }
    }

    match min_value {
        Some(min) => Ok(Value::Float(min)),
        None => Ok(Value::Empty),
    }
}

#[cfg(test)]
mod tests {
    use crate::templates::test_utils::MockRow;
    use super::*;

    #[test]
    fn test_rolling_min_basic_case() {
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
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(10.0), "Expected min value to be 10.0");
    }

    #[test]
    fn test_rolling_min_with_empty_values() {
        let row = MockRow::from_values(vec![
            Value::Float(0.1),
            Value::Empty,
            Value::Float(0.3),
            Value::Empty,
            Value::Float(0.5),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(0.1), "Expected min value to be 0.1");
    }

    #[test]
    fn test_rolling_min_mixed_values() {
        let row = MockRow::from_values(vec![
            Value::Float(1.0),
            Value::Int(3),
            Value::Float(4.0),
            Value::Int(2),
            Value::Float(5.0),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(1.0), "Expected min value to be 1.0");
    }

    #[test]
    fn test_rolling_min_empty_columns() {
        let row = MockRow::from_values(vec![
            Value::Float(1.0),
            Value::Int(3),
            Value::Float(4.0),
        ]).into_boxed();
        let columns: Vec<usize> = vec![];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Empty, "Expected an Empty value when no columns are provided");
    }

    #[test]
    fn test_rolling_min_single_value() {
        let row = MockRow::from_values(vec![
            Value::Float(2.5),
        ]).into_boxed();
        let columns = vec![0];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(2.5), "Expected min value to be 2.5 when only one value is provided");
    }

    #[test]
    fn test_rolling_min_all_empty_values() {
        let row = MockRow::from_values(vec![
            Value::Empty,
            Value::Empty,
            Value::Empty,
        ]).into_boxed();
        let columns = vec![0, 1, 2];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Empty, "Expected an Empty value when all columns contain Empty values");
    }

    #[test]
    fn test_rolling_min_negative_and_positive_values() {
        let row = MockRow::from_values(vec![
            Value::Float(-10.0),
            Value::Float(12.0),
            Value::Float(-23.0),
            Value::Float(23.0),
            Value::Float(-16.0),
        ]).into_boxed();
        let columns = vec![0, 1, 2, 3, 4];
        let result = rolling_min(&row, &columns).unwrap();

        assert_eq!(result, Value::Float(-23.0), "Expected min value to be -23.0");
    }
}

