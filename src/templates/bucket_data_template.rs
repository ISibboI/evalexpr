use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};


pub struct BucketData {
    bucket_output_field_name: String,
    bucket_range_output_field_name: String,
    field_name_to_bucket: String,
    no_buckets: u8
}

impl BucketData {
    pub fn new(field_to_bucket: &str, mut no_buckets: u8) -> BucketData {
        if no_buckets == 0 {
            no_buckets = 1;
        }
        let bucket_field_name = format!("{}bucket", field_to_bucket);
        let bucket_range_field_name = format!("{}bucketrange", field_to_bucket);
        BucketData {
            bucket_range_output_field_name: bucket_range_field_name,
            bucket_output_field_name: bucket_field_name,
            field_name_to_bucket: field_to_bucket.to_owned(),
            no_buckets
        }
    }
}
impl CompiledTransposeCalculationTemplate for BucketData {
    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            (self.bucket_range_output_field_name.clone(), ValueType::String),
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
        let mut transpose_value_to_field_value_map: HashMap<Value, Value> = HashMap::new();
        let mut value_to_bucket_map: HashMap<Value, u8> = HashMap::new();
        let mut min_values_for_bucket: HashMap<u8, Value> = HashMap::new();
        let mut max_values_for_bucket: HashMap<u8, Value> = HashMap::new();

        // Populate transpose_value_to_field_value_map
        for transpose_value in ordered_transpose_values {
            let field_to_bucket = row.get_value(&generate_column_name(&self.field_name_to_bucket, transpose_value))?;
            transpose_value_to_field_value_map.insert(transpose_value.clone(), field_to_bucket);
        }

        // Calculate buckets and populate value_to_bucket_map
        let num_buckets = self.no_buckets;
        let mut sorted_field_values: Vec<_> = transpose_value_to_field_value_map.values().cloned().collect();
        sorted_field_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));  // Sort in ascending order

        for (i, field_value) in sorted_field_values.iter().enumerate() {
            let bucket = (i * num_buckets as usize) / sorted_field_values.len();
            let bucket_u8 = bucket as u8;

            value_to_bucket_map.entry(field_value.clone()).or_insert(bucket_u8);

            min_values_for_bucket
                .entry(bucket_u8)
                .and_modify(|min_value| if field_value < min_value { *min_value = field_value.clone(); })
                .or_insert(field_value.clone());

            max_values_for_bucket
                .entry(bucket_u8)
                .and_modify(|max_value| if field_value > max_value { *max_value = field_value.clone(); })
                .or_insert(field_value.clone());
        }

        // Set bucket values in the row
        for (transpose_value, field_value) in transpose_value_to_field_value_map.iter() {
            if let Some(bucket) = value_to_bucket_map.get(field_value) {
                row.set_value(&generate_column_name(&self.bucket_output_field_name, transpose_value), Value::Int(*bucket as i64));
                let bucket_range = format!(
                    "{} to {}",
                    min_values_for_bucket.get(bucket).unwrap_or(&Value::Empty),
                    max_values_for_bucket.get(bucket).unwrap_or(&Value::Empty)
                );
                row.set_value(&generate_column_name(&self.bucket_range_output_field_name, transpose_value), Value::String(bucket_range))?;
            }
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::templates::test_utils::MockRow;

    // Mock implementation of BoxedOperatorRowTrait for testing purposes


    #[test]
    fn test_commit_row_basic() {
        // Create a mock row with initial values
        let mut row = MockRow::new();
        let field_to_bucket = "price";
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();

        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        bucket_data.commit_row(&mut row, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("bucket_date1").unwrap(), Value::Int(0));
        assert_eq!(row.get_value("bucketRange_date1").unwrap(), Value::String("foo".to_string()));
        assert_eq!(row.get_value("bucket_date2").unwrap(), Value::Int(2));
        assert_eq!(row.get_value("bucketRange_date2").unwrap(), Value::String("foo".to_string()));
        assert_eq!(row.get_value("bucket_date3").unwrap(), Value::Int(1));
        assert_eq!(row.get_value("bucketRange_date3").unwrap(), Value::String("foo".to_string()));
    }

    #[test]
    fn test_commit_row_with_equal_values() {
        // Create a mock row with identical values
        let mut row = MockRow::new();
        let field_to_bucket = "price";
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date1".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date2".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();

        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        bucket_data.commit_row(&mut row, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("bucket_date1").unwrap(), Value::Int(0));
        assert_eq!(row.get_value("bucket_date2").unwrap(), Value::Int(0));
        assert_eq!(row.get_value("bucket_date3").unwrap(), Value::Int(0));
    }


    #[test]
    fn test_commit_row_with_varied_values() {
        // Create a mock row with varied values
        let mut row = MockRow::new();
        let field_to_bucket = "price";
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();

        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };

        // Ordered transpose values (these should correspond to the field names)
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        bucket_data.commit_row(&mut row, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("bucket_date1").unwrap(), Value::Int(0));
        assert_eq!(row.get_value("bucket_date2").unwrap(), Value::Int(2));
        assert_eq!(row.get_value("bucket_date3").unwrap(), Value::Int(1));
    }

}
