use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, IntType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder, TransposeColumnIndex, TransposeColumnIndexHolder};
use crate::Error::CustomError;
use crate::templates::test_utils::MockIndexHolder;
use crate::templates::utils::{get_value_indirect, set_value_indirect};


fn to_bucket_field_name(field_name: &str) -> String {
    format!("{}bucket", field_name)
}

fn to_bucket_range_field_name(field_name: &str) -> String {
    format!("{}bucketRange", field_name)
}
pub struct BucketData {
    fields_to_bucket: Vec<String>,
    no_buckets: u8,
}

impl BucketData {
    pub fn new(fields_to_bucket: Vec<&str>, mut no_buckets: u8) -> BucketData {
        if no_buckets == 0 {
            no_buckets = 1;
        }
        BucketData {
            fields_to_bucket: fields_to_bucket.iter().map(|fld| fld.to_string()).collect(),
            no_buckets,
        }
    }
}
impl CompiledTransposeCalculationTemplate for BucketData {
    fn schema(&self) -> HashMap<String, ValueType> {
        self.fields_to_bucket.iter().map(|field| {
            let bucket_field_name = to_bucket_field_name(field);
            let bucket_range_field_name = to_bucket_range_field_name(field);
            vec![
                (bucket_field_name.clone(), ValueType::Int),
                (bucket_range_field_name.clone(), ValueType::String),
            ]
        }).flatten().collect()
    }
    fn dependencies(&self) -> Vec<String> {
        self.fields_to_bucket.clone()
    }
    fn commit_row(
        &self,
        row: &mut BoxedOperatorRowTrait,
        indexes: &BoxedTransposeColumnIndexHolder,
        ordered_transpose_values: &[Value],
        cycle_epoch: usize,
    ) -> Result<(), Error> {

        // Maps to hold value-to-bucket and range information

        let values = &row.get_values()?;
        let mut output_values = vec![Value::Empty; values.len()];
        let mut modified_columns = vec![];

        for field in &self.fields_to_bucket {
            let mut value_to_bucket_map: BTreeMap<Value, Vec<usize>> = BTreeMap::new();
            let mut min_values_for_bucket: HashMap<u8, Value> = HashMap::new();
            let mut max_values_for_bucket: HashMap<u8, Value> = HashMap::new();

            // Get column indexes
            let field_to_bucket_index = indexes.get_index_vec(field.clone())?;
            let bucket_range_index = indexes.get_index_vec(to_bucket_range_field_name(field))?;
            let bucket_index = indexes.get_index_vec(to_bucket_field_name(&field))?;

            // Populate transpose_value_to_field_value_map
            for (idx, transpose_value) in ordered_transpose_values.iter().enumerate() {
                let field_to_bucket = get_value_indirect(values, &field_to_bucket_index, idx)?;
                if field_to_bucket == &Value::Empty {
                    continue;
                }
                value_to_bucket_map.entry(field_to_bucket.clone()).or_default().push(idx);
            }

            let num_buckets = self.no_buckets;
            let no_values_in_value_to_bucket_map = value_to_bucket_map.len();
            for (idx, (bucket_value, transpose_columns_for_value)) in value_to_bucket_map.iter().enumerate() {
                let bucket = ((idx * num_buckets as usize) / no_values_in_value_to_bucket_map) as u8;

                for val in transpose_columns_for_value {
                    set_value_indirect(&mut output_values, &mut modified_columns, &bucket_index, *val, Value::Int((num_buckets - bucket) as IntType))?;
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
                    set_value_indirect(&mut output_values, &mut modified_columns, &bucket_range_index, *val, Value::String(bucket_range.clone().into()))?;
                }
            }
        }
        row.set_values_for_columns(modified_columns, output_values)?;
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
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];
        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            fields_to_bucket: vec![field_to_bucket.to_string()],
            no_buckets: 3,
        };

        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket, &"date1".into()), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date2".into()), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date3".into()), Value::Float(2.0)).unwrap();


        // Ordered transpose values (these should correspond to the field names)

        // Call commit_row and check the results
        let mut operator_row = BoxedOperatorRowTrait::new(row);
        let mut mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(operator_row.get_value("pricebucket__date1").unwrap(), Value::Int(3));
        assert_eq!(operator_row.get_value("pricebucketRange__date1").unwrap(), "1 to 1".into());
        assert_eq!(operator_row.get_value("pricebucket__date2").unwrap(), Value::Int(1));
        assert_eq!(operator_row.get_value("pricebucketRange__date2").unwrap(), "3 to 3".into());
        assert_eq!(operator_row.get_value("pricebucket__date3").unwrap(), Value::Int(2));
        assert_eq!(operator_row.get_value("pricebucketRange__date3").unwrap(), "2 to 2".into());
    }

    fn create_mock_index(ordered_transpose_values: &Vec<Value>, bucket_data: &BucketData) -> MockIndexHolder {
        let mut mock_index = MockIndexHolder::new();
        for field in &bucket_data.fields_to_bucket {
            mock_index.register_index(field.to_string(), &ordered_transpose_values);
            mock_index.register_index(to_bucket_field_name(field), &ordered_transpose_values);
            mock_index.register_index(to_bucket_range_field_name(field), &ordered_transpose_values);
        }
        mock_index
    }

    #[test]
    fn test_commit_row_with_equal_values() {
        // Create a mock row with identical values
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];
        let field_to_bucket = "price";
        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            fields_to_bucket: vec![field_to_bucket.to_string()],
            no_buckets: 3,
        };
        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket, &"date1".to_owned().into()), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date2".to_owned().into()), Value::Float(2.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date3".to_owned().into()), Value::Float(2.0)).unwrap();
        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        let mut mock_index = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut row, &mock_index, &ordered_transpose_values, 0).unwrap();
        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("pricebucket__date1").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("pricebucket__date2").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("pricebucket__date3").unwrap(), Value::Int(3));
    }


    #[test]
    fn test_commit_row_with_varied_values() {
        // Create a mock row with varied values
        let field_to_bucket = "price";
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        // Instantiate BucketData with dummy values
        let bucket_data = BucketData {
            fields_to_bucket: vec![field_to_bucket.to_string()],
            no_buckets: 3,
        };
        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        // Ordered transpose values (these should correspond to the field names)

        // Call commit_row and check the results
        let mut row = BoxedOperatorRowTrait::new(row);
        row.set_value(&generate_column_name(field_to_bucket, &"date1".to_owned().into()), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date2".to_owned().into()), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date3".to_owned().into()), Value::Float(2.0)).unwrap();

        let mut index = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut row, &index, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value("pricebucket__date1").unwrap(), Value::Int(3));
        assert_eq!(row.get_value("pricebucket__date2").unwrap(), Value::Int(1));
        assert_eq!(row.get_value("pricebucket__date3").unwrap(), Value::Int(2));
    }
}
