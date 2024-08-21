use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, IntType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder, TransposeColumnIndex, TransposeColumnIndexHolder};
use crate::templates::test_utils::MockIndexHolder;

pub struct BucketData {
    bucket_range_output_field_name: String,
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
        indexes: &BoxedTransposeColumnIndexHolder,
        ordered_transpose_values: &[Value],
        cycle_epoch: usize
    ) -> Result<(), Error> {

        // Maps to hold value-to-bucket and range information
        let mut transpose_value_to_field_value_map: BTreeMap<Value, Value> = BTreeMap::new();
        let mut value_to_bucket_map: BTreeMap<Value, Vec<usize>> = BTreeMap::new();
        let mut min_values_for_bucket: HashMap<u8, Value> = HashMap::new();
        let mut max_values_for_bucket: HashMap<u8, Value> = HashMap::new();

        // Get column indexes
        let field_to_bucket_index = indexes.get_index_for_column(self.field_name_to_bucket.clone())?;
        let bucket_range_index = indexes.get_index_for_column(self.bucket_range_output_field_name.clone())?;
        let bucket_index = indexes.get_index_for_column(self.bucket_output_field_name.clone())?;

        // Populate transpose_value_to_field_value_map
        for (idx, transpose_value) in ordered_transpose_values.iter().enumerate() {
            let field_to_bucket = row.get_value_for_column(field_to_bucket_index.col_idx(idx)?)?;
            if  field_to_bucket == Value::Empty {
                continue;
            }
            value_to_bucket_map.entry(field_to_bucket).or_default().push(idx);
        }

        // // Calculate buckets and populate value_to_bucket_map
        // let num_buckets = self.no_buckets;
        // for (i, (transpose_value, field_value)) in transpose_value_to_field_value_map.iter().enumerate() {
        //     if field_value == &Value::Empty {
        //         continue;
        //     }
        // 
        //     // Calculate bucket index
        //     let bucket = (i * num_buckets as usize) / transpose_value_to_field_value_map.len();
        //     let bucket_u8 = bucket as u8;
        // 
        //     // Assign field_value to a bucket if not already assigned
        //     value_to_bucket_map.entry(field_value.clone()).or_insert(bucket_u8);
        // 
        //     // Update min and max values for each bucket
        //     min_values_for_bucket
        //         .entry(bucket_u8)
        //         .and_modify(|min_value| if field_value < min_value { *min_value = field_value.clone(); })
        //         .or_insert(field_value.clone());
        // 
        //     max_values_for_bucket
        //         .entry(bucket_u8)
        //         .and_modify(|max_value| if field_value > max_value { *max_value = field_value.clone(); })
        //         .or_insert(field_value.clone());
        // }
        // 
        // // Set bucket and range values in the row
        // let no_buckets = value_to_bucket_map.len();
        let num_buckets = self.no_buckets;
        let no_values_in_value_to_bucket_map = value_to_bucket_map.len();
        for (idx, (bucket_value, transpose_columns_for_value)) in value_to_bucket_map.iter().enumerate() {

            let bucket = ((idx * num_buckets as usize) / no_values_in_value_to_bucket_map) as u8;

            for val in transpose_columns_for_value {
                let bucket_output_index = bucket_index.col_idx(*val)?;
                row.set_value_for_column(bucket_output_index, Value::Int((num_buckets - bucket) as IntType))?;
            }
            min_values_for_bucket
                    .entry(bucket)
                    .and_modify(|min_value| if bucket_value < min_value { *min_value = bucket_value.clone(); })
                    .or_insert(bucket_value.clone());
            max_values_for_bucket
                    .entry(bucket)
                    .and_modify(|max_value| if bucket_value > max_value { *max_value = bucket_value.clone(); })
                    .or_insert(bucket_value.clone());

        }

        for (idx, (bucket_value, transpose_columns_for_value)) in value_to_bucket_map.iter().enumerate() {

            let bucket = ((idx * num_buckets as usize) / no_values_in_value_to_bucket_map) as u8;
            let bucket_range = format!(
                "{} to {}",
                min_values_for_bucket.get(&bucket).unwrap_or(&Value::Empty),
                max_values_for_bucket.get(&bucket).unwrap_or(&Value::Empty)
            );
            for val in transpose_columns_for_value {
                let bucket_range_index = bucket_range_index.col_idx(*val)?;
                row.set_value_for_column(bucket_range_index, Value::String(bucket_range.clone()))?;
            }
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::templates::test_utils::{MockIndexHolder, MockRow};

    // Mock implementation of BoxedOperatorRowTrait for testing purposes


    #[test]
    fn test_commit_row_basic() {
        // Create a mock row with initial values
  
        let field_to_bucket = "price";

        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];
        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };

        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket,&Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();

 
        // Ordered transpose values (these should correspond to the field names)
   
        // Call commit_row and check the results
        let mut operator_row = BoxedOperatorRowTrait::new(row);
        let mut mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut operator_row,&mock_index_holder, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(operator_row.get_value("bucket__date1").unwrap(), Value::Int(3));
        assert_eq!(operator_row.get_value("bucketRange__date1").unwrap(), Value::String("1 to 1".to_string()));
        assert_eq!(operator_row.get_value("bucket__date2").unwrap(), Value::Int(1));
        assert_eq!(operator_row.get_value("bucketRange__date2").unwrap(), Value::String("3 to 3".to_string()));
        assert_eq!(operator_row.get_value("bucket__date3").unwrap(), Value::Int(2));
        assert_eq!(operator_row.get_value("bucketRange__date3").unwrap(), Value::String("2 to 2".to_string()));
    }

    fn create_mock_index(ordered_transpose_values: &Vec<Value>, bucket_data: &BucketData) -> MockIndexHolder {
        let mut mock_index = MockIndexHolder::new();
        mock_index.register_index(bucket_data.field_name_to_bucket.clone(), &ordered_transpose_values);
        mock_index.register_index(bucket_data.bucket_output_field_name.clone(), &ordered_transpose_values);
        mock_index.register_index(bucket_data.bucket_range_output_field_name.clone(), &ordered_transpose_values);
        mock_index
    }

    #[test]
    fn test_commit_row_with_equal_values() {
        // Create a mock row with identical values
        let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];
        let field_to_bucket = "price";
        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };
        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date1".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date2".to_owned())), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();
        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        let mut mock_index = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut row,&mock_index, &ordered_transpose_values, 0).unwrap();
        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("bucket__date1").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("bucket__date2").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("bucket__date3").unwrap(), Value::Int(3));
    }


    #[test]
    fn test_commit_row_with_varied_values() {
        // Create a mock row with varied values
        let field_to_bucket = "price";
           let ordered_transpose_values = vec![
            Value::String("date1".to_string()),
            Value::String("date2".to_string()),
            Value::String("date3".to_string()),
        ];

        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            field_name_to_bucket: field_to_bucket.to_owned(),
            bucket_output_field_name: "bucket".to_string(),
            no_buckets: 3,
            bucket_range_output_field_name: "bucketRange".to_string(),
        };
        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        // Ordered transpose values (these should correspond to the field names)
    
        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date1".to_owned())), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date2".to_owned())), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &Value::String("date3".to_owned())), Value::Float(2.0)).unwrap();

        let mut index = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut row,&index, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("bucket__date1").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("bucket__date2").unwrap(), Value::Int(1));
        assert_eq!(row.get_value("bucket__date3").unwrap(), Value::Int(2));
    }

}
