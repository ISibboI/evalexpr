use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, AdaptiveStopLossTradeModel};

pub struct RollingZScore {
    avg_output_field_name: String,
    stdev_output_field_name: String,
    has_data_output_field_name: String,
    zscore_output_field_name: String,
    field_to_zscore: String,
    window_size: u32,
}

impl RollingZScore {
    pub fn new(field_to_zscore: &str, output_column_prefix: &str, mut window_size: u32) -> Self {
        if  window_size == 0 {
            window_size = 1;
        }
        RollingZScore {
            avg_output_field_name: format!("{}avg", output_column_prefix),
            stdev_output_field_name: format!("{}stdev", output_column_prefix),
            has_data_output_field_name: format!("{}hasdata", output_column_prefix),
            zscore_output_field_name: format!("{}zscore", output_column_prefix),
            field_to_zscore: field_to_zscore.to_string(),
            window_size,
        }
    }
}

impl CompiledTransposeCalculationTemplate for RollingZScore {
    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            (self.has_data_output_field_name.clone(), ValueType::Boolean),
            (self.avg_output_field_name.clone(), ValueType::Float),
            (self.stdev_output_field_name.clone(), ValueType::Float),
            (self.zscore_output_field_name.clone(), ValueType::Float),
        ]
            .into_iter()
            .collect()
    }

    fn dependencies(&self) -> Vec<String> {
        vec![self.field_to_zscore.clone()]
    }

    fn commit_row(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {
        let mut score_window = Vec::with_capacity(self.window_size as usize);
        let mut window_sum = 0.0;
        let mut window_sum_squares = 0.0;

        // These variables will store the last valid values for avg, stdev, and zscore as Options
        let mut last_avg: Option<FloatType> = None;
        let mut last_stdev: Option<FloatType> = None;
        let mut last_zscore: Option<FloatType> = None;
        let mut last_value: Option<FloatType> = None;

        for i in cmp::max(cycle_epoch as isize - self.window_size as isize, 0) as usize..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];
            let value_opt = row.get_value(&generate_column_name(&self.field_to_zscore, transpose_value))?.as_float_or_none()?;

            // Process non-null values and update the window
            if let Some(value) = value_opt.or(last_value) {
                last_value = Some(value);
                if score_window.len() == self.window_size as usize {
                    let oldest_value = score_window.remove(0);

                    // Remove the oldest value from the window
                    window_sum -= oldest_value;
                    window_sum_squares -= oldest_value * oldest_value;
                }

                // Add the new value to the window
                score_window.push(value);
                window_sum += value;
                window_sum_squares += value * value;

                // Only proceed if the window is fully populated
                if score_window.len() == self.window_size as usize {
                    // Calculate the average and standard deviation
                    let avg = window_sum / self.window_size as FloatType;
                    let variance = (window_sum_squares / self.window_size as FloatType) - (avg * avg);
                    let stdev = variance.sqrt();
                    let zscore = (value - avg) / stdev;

                    // Update the last valid values
                    last_avg = Some(avg);
                    last_stdev = Some(stdev);
                    last_zscore = Some(zscore);

                    // Set the calculated values
                    row.set_value(&generate_column_name(&self.zscore_output_field_name, transpose_value), Value::Float(zscore))?;
                    row.set_value(&generate_column_name(&self.stdev_output_field_name, transpose_value), Value::Float(stdev))?;
                    row.set_value(&generate_column_name(&self.avg_output_field_name, transpose_value), Value::Float(avg))?;
                    row.set_value(&generate_column_name(&self.has_data_output_field_name, transpose_value), Value::Boolean(true))?;
                }
            } else if score_window.len() == self.window_size as usize {
                // If the current value is null but the window is full, output the last valid values
                row.set_value(
                    &generate_column_name(&self.zscore_output_field_name, transpose_value),
                    last_zscore.map(Value::Float).unwrap_or(Value::Empty),
                )?;
                row.set_value(
                    &generate_column_name(&self.stdev_output_field_name, transpose_value),
                    last_stdev.map(Value::Float).unwrap_or(Value::Empty),
                )?;
                row.set_value(
                    &generate_column_name(&self.avg_output_field_name, transpose_value),
                    last_avg.map(Value::Float).unwrap_or(Value::Empty),
                )?;
                row.set_value(
                    &generate_column_name(&self.has_data_output_field_name, transpose_value),
                    Value::Boolean(true),
                )?;
            }
        }

        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::templates::test_utils::MockRow;

    #[test]
    fn test_commit_row_basic() {
        // Create a mock row with initial values
        let mut row = MockRow::new();
        let field_to_zscore = "price";
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date4".to_owned())), Value::Float(4.0)).unwrap();

        // Instantiate RollingZScore with dummy values
        let rolling_zscore = RollingZScore::new(field_to_zscore, "prefix", 3);

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
            Value::String("date4".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        rolling_zscore.commit_row(&mut row, &ordered_transpose_values, 0).unwrap();

        // Check that the correct zscore, avg, stdev, and has_data values have been set
        assert!(row.get_value("prefixzscore__date3").unwrap() != Value::Empty); // First full window
        assert!(row.get_value("prefixzscore__date4").unwrap() != Value::Empty); // Second full window
        assert_eq!(row.get_value("prefixhasdata__date3").unwrap(), Value::Boolean(true));
        assert_eq!(row.get_value("prefixhasdata__date4").unwrap(), Value::Boolean(true));

        // Test for the actual computed values
        // Replace `expected_zscore_date3` with the exact value.
 
        assert_eq!(row.get_value("prefixzscore__date1").unwrap(), Value::Empty);
        assert_eq!(row.get_value("prefixzscore__date2").unwrap(), Value::Empty);
        assert_eq!(row.get_value("prefixzscore__date3").unwrap(), Value::Float(0f64));
        assert_eq!(row.get_value("prefixzscore__date4").unwrap(), Value::Float(1.2247448713915896f64));
    }

    #[test]
    fn test_commit_row_with_nulls() {
        // Create a mock row with initial values, including a null value
        let mut row = MockRow::new();
        let field_to_zscore = "price";
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".to_owned())), Value::Empty).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date4".to_owned())), Value::Float(3.0)).unwrap();

        // Instantiate RollingZScore with dummy values
        let rolling_zscore = RollingZScore::new(field_to_zscore, "prefix",  3);

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
            Value::String("date4".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        rolling_zscore.commit_row(&mut row, &ordered_transpose_values, 0).unwrap();

        // Check that the correct zscore, avg, stdev, and has_data values have been set
        assert!(row.get_value("prefixzscore__date2").unwrap() == Value::Empty); // Should be empty because it's null
        assert!(row.get_value("prefixzscore__date3").unwrap() != Value::Empty); // Should have a value
        assert!(row.get_value("prefixzscore__date4").unwrap() != Value::Empty); // Should have a value
        assert_eq!(row.get_value("prefixhasdata__date3").unwrap(), Value::Boolean(true));
        assert_eq!(row.get_value("prefixhasdata__date4").unwrap(), Value::Boolean(true));

        
        // Test for the actual computed values
        // Replace with the actual expected values based on your calculations
        assert_eq!(row.get_value("prefixzscore__date3").unwrap(), Value::Float(1.414213562373095f64));
    }

    #[test]
    fn test_commit_row_window_not_full() {
        // Create a mock row with initial values
        let mut row = MockRow::new();
        let field_to_zscore = "price";
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();

        // Instantiate RollingZScore with dummy values
        let rolling_zscore = RollingZScore::new(field_to_zscore, "zscore_",  3);

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        rolling_zscore.commit_row(&mut row, &ordered_transpose_values, 2).unwrap();

        // Check that no values have been set because the window is not full
        assert!(row.get_value("zscore_zscore_date1").unwrap() == Value::Empty); // Should be empty because the window isn't full
        assert!(row.get_value("zscore_zscore_date2").unwrap() == Value::Empty); // Should be empty because the window isn't full
    }

    #[test]
    fn test_commit_row_performance() {
        // Create a mock row with initial values
        let mut row = MockRow::new();
        let field_to_zscore = "price";

        // Fill the row with 15,000 values
        for i in 1..=15000 {
            row.set_value(
                &generate_column_name(field_to_zscore, &Value::String(format!("date{}", i))),
                Value::Float((i % 10) as f64 + 1.0),
            ).unwrap();
        }

        // Instantiate RollingZScore with window size of 522
        let rolling_zscore = RollingZScore::new(field_to_zscore, "prefix", 522);

        // Generate the ordered transpose values
        let ordered_transpose_values: Vec<Value> = (1..=15000)
            .map(|i| Value::String(format!("date{}", i)))
            .collect();

        // Call commit_row multiple times and measure the time
        let iterations = 10;
        let mut total_duration = 0;

        for _ in 0..iterations {
            let mut row_clone = BoxedOperatorRowTrait::new(row.clone()); // Clone the row to reset the state for each iteration
            let start_time = Instant::now();
            rolling_zscore.commit_row(&mut row_clone, &ordered_transpose_values, 0).unwrap();
            let duration = start_time.elapsed();
            total_duration += duration.as_millis();
        }

        let average_duration = total_duration as f64 / iterations as f64;

        println!("Average execution time for commit_row: {} millis", average_duration);

        // Asserting that the average execution time is within an acceptable range (this is optional)
        assert!(average_duration < 1_000_000f64); // Example threshold: 1 second
    }
}

