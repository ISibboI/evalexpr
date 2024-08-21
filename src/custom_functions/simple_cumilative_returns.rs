use crate::{BoxedOperatorRowTrait, Error, OperatorRowTrait, Value};

pub fn simple_cumulative_returns(row: &BoxedOperatorRowTrait, columns: &[usize]) -> Result<Value, Error> {
    if columns.is_empty() {
        return Ok(Value::Empty);
    }

    let mut cumulative_return = 1.0f64;
    let mut last_val = None;
    for &col_index in columns {
        if let Some(val) = row.get_value_for_column(col_index).ok() {
            last_val = Some(val.clone());
            match val {
                Value::Float(val) => {
                    cumulative_return *= 1.0 + val;
                }
                Value::Int(val) => {
                    cumulative_return *= 1.0 + (val as f64);
                }
                Value::Empty => {
                    match last_val {
                        None => {}
                        Some(val) => {
                            match val {
                                Value::Float(val) => {
                                    cumulative_return *= 1.0 + val;
                                },
                                Value::Int(val) => {
                                    cumulative_return *= 1.0 + (val as f64);
                                },
                                Value::Empty => {}
                                _ => return Err(Error::NonNumericType),
                            }
                        }
                    }
                }
                _ => return Err(Error::NonNumericType),
            }
        }
    }

    Ok(Value::Float(cumulative_return - 1.0))
}

#[cfg(test)]
mod tests {
    use crate::templates::test_utils::{MockIndexHolder, MockRow};
    use super::*;

    #[test]
    fn test_simple_cumulative_returns_normal_operation() {
        let mock_holder = MockIndexHolder::new();
        let row = MockRow::from_values(vec![Value::Float(0.1), Value::Float(0.2), Value::Float(0.3)],&mock_holder).into_boxed();
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 1.2 * 1.3 - 1.0));
    }

    #[test]
    fn test_simple_cumulative_returns_partial_data() {
        let mock_holder = MockIndexHolder::new();
        let row = MockRow::from_values(vec![Value::Float(0.1), Value::Empty, Value::Float(0.3)],&mock_holder).into_boxed();
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 1.3 - 1.0));
    }

    #[test]
    fn test_simple_cumulative_returns_empty_input() {
        let mock_holder = MockIndexHolder::new();
        let row= MockRow::from_values(vec![],&mock_holder).into_boxed();
        let columns: Vec<usize> = vec![];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Empty);
    }

    #[test]
    fn test_simple_cumulative_returns_no_valid_columns() {
        let mock_holder: MockIndexHolder = MockIndexHolder::new();
        let row = MockRow::from_values(vec![Value::Empty, Value::Empty],&mock_holder).into_boxed();
        let columns = vec![0, 1];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(0.0)); // Assuming no valid data means a return of 0.0
    }

    #[test]
    fn test_simple_cumulative_returns_mixed_data_types() {
        let mock_holder: MockIndexHolder = MockIndexHolder::new();
        let row = MockRow::from_values(vec![Value::Float(0.1), Value::Int(2), Value::Float(-0.1)],&mock_holder).into_boxed();
        let columns = vec![0, 1, 2];
        let result = simple_cumulative_returns(&row, &columns).unwrap();
        assert_eq!(result, Value::Float(1.1 * 3.0 * 0.9 - 1.0));
    }
}
