use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, AdaptiveStopLossTradeModel};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder, TransposeColumnIndex, TransposeColumnIndexHolder};
use crate::templates::utils::{get_value_indirect, set_value_indirect};

pub struct RollingZScore {
    fields_to_zscore: Vec<String>,
    window_size: u32
}

impl RollingZScore {
    pub fn new(fields_to_zscore: Vec<&str>, mut window_size: u32) -> Self {
        if  window_size == 0 {
            window_size = 1;
        }
        RollingZScore {
            fields_to_zscore: fields_to_zscore.iter().map(|fld|fld.to_string()).collect(),
            window_size,
        }
    }

  
}
fn to_has_data_output_field_name(field: &String) -> String {
    format!("{}hasdata", field)
}

fn to_avg_output_field_name(field: &str) -> String {
    format!("{}avg", field)
}
fn to_std_dev_output_field_name(field: &str) -> String {
    format!("{}stdev", field)
}fn to_zscore_output_field_name(field: &str) -> String {
    format!("{}zscore", field)
}
impl CompiledTransposeCalculationTemplate for RollingZScore {

    fn schema(&self) -> HashMap<String, ValueType> {
        let mut result = vec![];
        for field in &self.fields_to_zscore {
            result.push((to_avg_output_field_name(field), ValueType::Float));
            result.push((to_std_dev_output_field_name(field), ValueType::Float));
            result.push((to_has_data_output_field_name(field), ValueType::Boolean));
            result.push((to_zscore_output_field_name(field), ValueType::Float));
        }
        result
            .into_iter()
            .collect()
    }

    fn dependencies(&self) -> Vec<String> {
        self.fields_to_zscore.clone()
    }

    fn commit_row(&self, row: &mut BoxedOperatorRowTrait,indexes: &BoxedTransposeColumnIndexHolder, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {
        let mut score_window = Vec::with_capacity(self.window_size as usize);
        let mut window_sum = 0.0;
        let mut window_sum_squares = 0.0;

        // These variables will store the last valid values for avg, stdev, and zscore as Options
        

        let row_values = &row.get_values()?;
        let mut output_values = vec![Value::Empty; row_values.len()];
        let mut modified_columns = vec![];


        for field in &self.fields_to_zscore {

            let mut last_avg: Option<FloatType> = None;
            let mut last_stdev: Option<FloatType> = None;
            let mut last_zscore: Option<FloatType> = None;
            let mut last_value: Option<FloatType> = None;
            
            let zscore_output_field_name = to_zscore_output_field_name(field);
            let stdev_output_field_names = to_std_dev_output_field_name(field);
            let avg_output_field_names = to_avg_output_field_name(field);
            let has_data_output_field_names = to_has_data_output_field_name(field);
            
            let zscore_output_index = &indexes.get_index_vec(zscore_output_field_name.clone())?;
            let stdev_index = &indexes.get_index_vec(stdev_output_field_names.clone())?;
            let avg_index = &indexes.get_index_vec(avg_output_field_names.clone())?;
            let has_data_index = &indexes.get_index_vec(has_data_output_field_names.clone())?;
            let input_field_index = &indexes.get_index_vec(field.clone())?;


            for i in cmp::max(cycle_epoch as isize - self.window_size as isize, 0) as usize..ordered_transpose_values.len() {
                let transpose_value = &ordered_transpose_values[i];
                let value_opt = get_value_indirect(row_values, input_field_index, i)?.as_float_or_none()?;

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


                        set_value_indirect(&mut  output_values,&mut  modified_columns,zscore_output_index,i,Value::Float(zscore))?;
                        set_value_indirect(&mut  output_values,&mut  modified_columns,stdev_index,i,Value::Float(stdev))?;
                        set_value_indirect(&mut  output_values,&mut  modified_columns,avg_index,i,Value::Float(avg))?;
                        set_value_indirect(&mut  output_values,&mut  modified_columns,has_data_index,i,Value::Boolean(true))?;


                    }
                } else if score_window.len() == self.window_size as usize {
                    // If the current value is null but the window is full, output the last valid values
                    set_value_indirect(&mut  output_values,&mut  modified_columns,zscore_output_index,i,last_zscore.map(Value::Float).unwrap_or(Value::Empty))?;
                    set_value_indirect(&mut  output_values,&mut  modified_columns,stdev_index,i,last_stdev.map(Value::Float).unwrap_or(Value::Empty))?;
                    set_value_indirect(&mut  output_values,&mut  modified_columns,avg_index,i,last_avg.map(Value::Float).unwrap_or(Value::Empty))?;
                    set_value_indirect(&mut  output_values,&mut  modified_columns,has_data_index,i, last_zscore.map(|fl| Value::Boolean(true)).unwrap_or(Value::Empty))?;
                }
            }

        }
        
       
        row.set_values_for_columns(modified_columns,output_values)?;

        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::templates::test_utils::{MockIndexHolder, MockRow};

    #[test]
    fn test_commit_row_basic() {
        // Create a mock row with initial values

        let ordered_transpose_values = vec![
            Value::String("date1".into()),
            Value::String("date2".into()),
            Value::String("date3".into()),
            Value::String("date4".into()),
        ];

        let field_to_zscore = "price";
        let rolling_zscore = RollingZScore::new(vec![field_to_zscore],  3);
        
        let mock_index_column_holder  = create_index_holder(&ordered_transpose_values, &rolling_zscore);
        let mut row = MockRow::new(&mock_index_column_holder);
        
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".into())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".into())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date3".into())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date4".into())), Value::Float(4.0)).unwrap();

        // Instantiate RollingZScore with dummy values
  

        // Ordered transpose values (these should correspond to the field names)
    

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        rolling_zscore.commit_row(&mut row,&index_holder_trait, &ordered_transpose_values, 0).unwrap();

        // Check that the correct zscore, avg, stdev, and has_data values have been set
        assert!(row.get_value("pricezscore__date3").unwrap() != Value::Empty); // First full window
        assert!(row.get_value("pricezscore__date4").unwrap() != Value::Empty); // Second full window
        assert_eq!(row.get_value("pricehasdata__date3").unwrap(), Value::Boolean(true));
        assert_eq!(row.get_value("pricehasdata__date4").unwrap(), Value::Boolean(true));


        // Test for the actual computed values
        // Replace `expected_zscore_date3` with the exact value.
 
        assert_eq!(row.get_value("pricezscore__date1").unwrap(), Value::Empty);
        assert_eq!(row.get_value("pricezscore__date2").unwrap(), Value::Empty);
        assert_eq!(row.get_value("pricezscore__date3").unwrap(), Value::Float(0f64));
        assert_eq!(row.get_value("pricezscore__date4").unwrap(), Value::Float(1.2247448713915896f64));
    }

    fn create_index_holder(ordered_transpose_values: &Vec<Value>, rolling_zscore: &RollingZScore) -> MockIndexHolder {
        let mut mock_index_column_holder = MockIndexHolder::new();
        for field in &rolling_zscore.fields_to_zscore {
            mock_index_column_holder.register_index(field.to_string(), &ordered_transpose_values);
            mock_index_column_holder.register_index(to_zscore_output_field_name(&field), &ordered_transpose_values);
            mock_index_column_holder.register_index(to_avg_output_field_name(&field), &ordered_transpose_values);
            mock_index_column_holder.register_index(to_std_dev_output_field_name(&field), &ordered_transpose_values);
            mock_index_column_holder.register_index(to_has_data_output_field_name(&field), &ordered_transpose_values);
        }
      
        mock_index_column_holder
    }

    #[test]
    fn test_commit_row_with_nulls() {
        // Create a mock row with initial values, including a null value
        let field_to_zscore = "price";
        
        let rolling_zscore = RollingZScore::new(vec![field_to_zscore],   3);
        let ordered_transpose_values = vec![
            Value::String("date1".into()),
            Value::String("date2".into()),
            Value::String("date3".into()),
            Value::String("date4".into()),
        ];
        let mock_index_column_holder  = create_index_holder(&ordered_transpose_values, &rolling_zscore);
        let mut row = MockRow::new(&mock_index_column_holder);
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".into())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".into())), Value::Empty).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date3".into())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date4".into())), Value::Float(3.0)).unwrap();

        // Instantiate RollingZScore with dummy values

        // Ordered transpose values (these should correspond to the field names)


        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        rolling_zscore.commit_row(&mut row,&index_holder_trait, &ordered_transpose_values, 0).unwrap();

        // Check that the correct zscore, avg, stdev, and has_data values have been set
        assert!(row.get_value("pricezscore__date2").unwrap() == Value::Empty); // Should be empty because it's null
        assert!(row.get_value("pricezscore__date3").unwrap() != Value::Empty); // Should have a value
        assert!(row.get_value("pricezscore__date4").unwrap() != Value::Empty); // Should have a value
        assert_eq!(row.get_value("pricehasdata__date3").unwrap(), Value::Boolean(true));
        assert_eq!(row.get_value("pricehasdata__date4").unwrap(), Value::Boolean(true));

        
        // Test for the actual computed values
        // Replace with the actual expected values based on your calculations
        assert_eq!(row.get_value("pricezscore__date3").unwrap(), Value::Float(1.414213562373095f64));
    }

    #[test]
    fn test_commit_row_window_not_full() {
        
        // Create a mock row with initial values

        let field_to_zscore = "price";
        let rolling_zscore = RollingZScore::new(vec![field_to_zscore],   3);
        let ordered_transpose_values = vec![
            Value::String("date1".into()),
            Value::String("date2".into()),
        ];

        let mock_index_column_holder  = create_index_holder(&ordered_transpose_values, &rolling_zscore);
        let mut row = MockRow::new(&mock_index_column_holder);
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date1".into())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_zscore, &Value::String("date2".into())), Value::Float(3.0)).unwrap();

        // Instantiate RollingZScore with dummy values

        // Ordered transpose values (these should correspond to the field names)
    

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        rolling_zscore.commit_row(&mut row,&index_holder_trait, &ordered_transpose_values, 2).unwrap();

        // Check that no values have been set because the window is not full
        assert!(row.get_value("zscore_zscore_date1").unwrap() == Value::Empty); // Should be empty because the window isn't full
        assert!(row.get_value("zscore_zscore_date2").unwrap() == Value::Empty); // Should be empty because the window isn't full
    }

    #[test]
    fn test_commit_row_performance() {
        // Create a mock row with initial values

        let ordered_transpose_values: Vec<Value> = (1..=15000)
            .map(|i| Value::String(format!("date{}", i).into()))
            .collect();


        let field_to_zscore = "price";
        let rolling_zscore = RollingZScore::new(vec![field_to_zscore],  522);
        let mock_index_column_holder  = create_index_holder(&ordered_transpose_values, &rolling_zscore);
        let mut row = MockRow::new(&mock_index_column_holder);

        // Fill the row with 15,000 values
        for i in 1..=15000 {
            row.set_value(
                &generate_column_name(field_to_zscore, &Value::String(format!("date{}", i).into())),
                Value::Float((i % 10) as f64 + 1.0),
            ).unwrap();
        }

        // Instantiate RollingZScore with window size of 522

        // Generate the ordered transpose values
   

        // Call commit_row multiple times and measure the time
        let iterations = 10;
        let mut total_duration = 0;

        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        for _ in 0..iterations {
            let mut row_clone = BoxedOperatorRowTrait::new(row.clone()); // Clone the row to reset the state for each iteration
            let start_time = Instant::now();
            rolling_zscore.commit_row(&mut row_clone,&index_holder_trait, &ordered_transpose_values, 0).unwrap();
            let duration = start_time.elapsed();
            total_duration += duration.as_millis();
        }

        let average_duration = total_duration as f64 / iterations as f64;

        println!("Average execution time for commit_row: {} millis", average_duration);

        // Asserting that the average execution time is within an acceptable range (this is optional)
        assert!(average_duration < 1_000_000f64); // Example threshold: 1 second
    }
}

