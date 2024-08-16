use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};


pub struct BucketData {
    bucket_output_field_name: String,
    field_name_to_bucket: String,
    no_buckets: usize
}

impl BucketData {
    pub fn new(field_to_bucket: &str, no_buckets: u8) -> BucketData {
        if no_buckets == 0 {
            no_buckets = 1;
        }
        let bucket_field_name = format!("{}_bucket", field_to_bucket);
        BucketData {
            bucket_output_field_name: bucket_field_name,
            field_name_to_bucket: field_to_bucket,
            no_buckets
        }
    }
}
impl CompiledTransposeCalculationTemplate for BucketData {
    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            (self.bucket_output_field_name.clone(), ValueType::Int),
        ].iter().map(|(nm, val)| (nm.to_string(), *val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        vec![
            self.field_name_to_bucket.clone()
        ]
    }
    fn commit_row(
        &self,
        row: &mut BoxedOperatorRowTrait,
        ordered_transpose_values: &[Value],
        cycle_epoch: usize
    ) -> Result<(), Error> {
        // Maps transpose values to field values
        let mut transpose_value_to_field_value_map: HashMap<Value, Value> = HashMap::new();
        // Maps field values to their corresponding bucket
        let mut value_to_bucket_map: HashMap<Value, usize> = HashMap::new();

        // Populate transpose_value_to_field_value_map
        for i in 0..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];
            let field_to_bucket = row.get_value(&generate_column_name(&self.field_name_to_bucket, transpose_value))?.as_float_or_none()?;
            if let Some(field_value) = field_to_bucket {
                transpose_value_to_field_value_map.insert(transpose_value.clone(), field_value);
            }
        }

        // Calculate buckets and populate value_to_bucket_map
        let num_buckets = self.no_buckets;
        let mut sorted_field_values: Vec<_> = transpose_value_to_field_value_map.values().cloned().collect();
        sorted_field_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));

        for (i, field_value) in sorted_field_values.iter().enumerate() {
            let bucket = (i * num_buckets) / sorted_field_values.len();
            value_to_bucket_map.insert(field_value.clone(), bucket);
        }

        // Set bucket values in the row
        for (transpose_value, field_value) in transpose_value_to_field_value_map.iter() {
            if let Some(bucket) = value_to_bucket_map.get(field_value) {
                row.set_value(&generate_column_name(&self.bucket_output_field_name, transpose_value), Value::Int(*bucket as i64))?;
            }
        }
        Ok(())
    }

}

#[cfg(test)]
mod commit_row_tests {
    use super::*;
    use std::collections::HashMap;
    use crate::templates::test_utils::MockRow;

    #[test]
    fn test_commit_row_value_below_lower_band() -> Result<(), Error> {
        let monitor = ChannelMonitor::new(
            "lower_band",
            "upper_band",
            "value",
            "output",
        );

        let mut row = MockRow::new();
        let cycle_epoch = 0;
        let transpose_value = Value::String("02_01_2024".to_owned());
        let ordered_transpose_values = vec![transpose_value.clone()];

        row.insert_value(generate_column_name("lower_band", &transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &transpose_value), Value::Float(5.0));

        let mut row_trait = BoxedOperatorRowTrait::new(row);
        monitor.commit_row(&mut row_trait, &ordered_transpose_values, cycle_epoch)?;

        assert_eq!(row_trait.get_value(&generate_column_name("output_within_bounds", &transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_left_above", &transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_left_below", &transpose_value))?, Value::Boolean(true));

        Ok(())
    }

    #[test]
    fn test_commit_row_re_entered_above_after_being_below() -> Result<(), Error> {
        let monitor = ChannelMonitor::new(
            "lower_band",
            "upper_band",
            "value",
            "output",
        );

        let mut row = MockRow::new();
        let prev_transpose_value = Value::String("01_01_2024".to_owned());
        let current_transpose_value = Value::String("02_01_2024".to_owned());
        let ordered_transpose_values = vec![prev_transpose_value.clone(), current_transpose_value.clone()];

        // Previous epoch - below lower band

        row.insert_value(generate_column_name("lower_band", &prev_transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &prev_transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &prev_transpose_value), Value::Float(5.0));

        // Current epoch - within bounds, simulating a re-entry from below
        row.insert_value(generate_column_name("lower_band", &current_transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &current_transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &current_transpose_value), Value::Float(15.0));

        let mut row_trait = BoxedOperatorRowTrait::new(row);
        monitor.commit_row(&mut row_trait, &ordered_transpose_values, 0)?;

        assert_eq!(row_trait.get_value(&generate_column_name("output_within_bounds", &prev_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_left_below", &prev_transpose_value))?, Value::Boolean(true));
        assert_eq!(row_trait.get_value(&generate_column_name("output_left_above", &prev_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_above", &prev_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_below", &prev_transpose_value))?, Value::Boolean(false));


        assert_eq!(row_trait.get_value(&generate_column_name("output_within_bounds", &current_transpose_value))?, Value::Boolean(true));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_above", &current_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_below", &current_transpose_value))?, Value::Boolean(true));

        Ok(())
    }

    #[test]
    fn test_commit_row_re_entered_below_after_being_above() -> Result<(), Error> {
        let monitor = ChannelMonitor::new(
            "lower_band",
            "upper_band",
            "value",
            "output",
        );

        let mut row = MockRow::new();
        let prev_transpose_value = Value::String("01_01_2024".to_owned());
        let current_transpose_value = Value::String("02_01_2024".to_owned());
        let ordered_transpose_values = vec![prev_transpose_value.clone(), current_transpose_value.clone()];

        // Previous epoch - above upper band

        row.insert_value(generate_column_name("lower_band", &prev_transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &prev_transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &prev_transpose_value), Value::Float(25.0));

        // Current epoch - within bounds, simulating a re-entry from above
        row.insert_value(generate_column_name("lower_band", &current_transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &current_transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &current_transpose_value), Value::Float(15.0));

        let mut row_trait = BoxedOperatorRowTrait::new(row);
        monitor.commit_row(&mut row_trait, &ordered_transpose_values, 0)?;

        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_below", &current_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_above", &current_transpose_value))?, Value::Boolean(true));

        Ok(())
    }

}