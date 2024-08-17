use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};


pub struct BucketData {
    bucket_output_field_name: String,
    field_name_to_bucket: String,
    no_buckets: u8
}

impl BucketData {
    pub fn new(field_to_bucket: &str, mut no_buckets: u8) -> BucketData {
        if no_buckets == 0 {
            no_buckets = 1;
        }
        let bucket_field_name = format!("{}bucket", field_to_bucket);
        BucketData {
            bucket_output_field_name: bucket_field_name,
            field_name_to_bucket: field_to_bucket.to_owned(),
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
        let mut value_to_bucket_map: HashMap<Value, u8> = HashMap::new();

        // Populate transpose_value_to_field_value_map
        for i in 0..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];
            let field_to_bucket = row.get_value(&generate_column_name(&self.field_name_to_bucket, transpose_value))?;
            transpose_value_to_field_value_map.insert(transpose_value.clone(), field_to_bucket);
        }

        // Calculate buckets and populate value_to_bucket_map
        let num_buckets = self.no_buckets;
        let mut sorted_field_values: Vec<_> = transpose_value_to_field_value_map.values().cloned().collect();
        sorted_field_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));

        for (i, field_value) in sorted_field_values.iter().enumerate() {
            let bucket = (i * num_buckets as usize) / sorted_field_values.len();
            value_to_bucket_map.entry(field_value.clone()).or_insert(bucket as u8);
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
