use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, IntType, EvalexprResult};
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

pub  struct BucketSpec{
    pub field_to_bucket: String,
    pub range_stops: Vec<(Value)>,
}

impl BucketSpec {
    pub fn get_bucket(&self, value: &Value) -> u8 {
        for (idx, range_stop) in self.range_stops.iter().enumerate() {
            if value <= range_stop {
                return idx as u8;
            }
        }
        (self.range_stops.len()) as u8
    }
}

pub struct BucketDataDiscrete {
    fields_to_bucket: Vec<BucketSpec>,
    pub generate_bucket_range: bool,
}


pub  fn bucket_rng(field: &str, mut range_stops: Vec<Value>) -> BucketSpec {
    range_stops.sort();
    BucketSpec {
        field_to_bucket: field.to_string(),
        range_stops,
    }
}

impl BucketDataDiscrete {
    pub fn new(fields_to_bucket: Vec<BucketSpec>, generate_bucket_range: bool) -> BucketDataDiscrete {
        BucketDataDiscrete {
            generate_bucket_range,
            fields_to_bucket,
        }
    }
}
impl CompiledTransposeCalculationTemplate for BucketDataDiscrete {
    fn schema(&self) -> HashMap<String, ValueType> {
        self.fields_to_bucket.iter().map(|field| {
            let bucket_field_name = to_bucket_field_name(&field.field_to_bucket);
            let bucket_range_field_name = to_bucket_range_field_name(&field.field_to_bucket);
            vec![
                (bucket_field_name.clone(), ValueType::Int),
                (bucket_range_field_name.clone(), ValueType::String),
            ]
        }).flatten().collect()
    }
    fn dependencies(&self) -> Vec<String> {
        self.fields_to_bucket.iter().map(|field| field.field_to_bucket.clone()).collect()
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

        for bucket_spec in &self.fields_to_bucket {
            let mut min_values_for_bucket: HashMap<u8, Value> = HashMap::new();
            let mut max_values_for_bucket: HashMap<u8, Value> = HashMap::new();
            let mut index_to_bucket_map: HashMap<usize, u8> = HashMap::new();
            // Get column indexes
            let field_name = bucket_spec.field_to_bucket.clone();
            let field_to_bucket_index = indexes.get_index_vec(field_name.clone())?;
            let bucket_range_index = indexes.get_index_vec(to_bucket_range_field_name(&field_name))?;
            let bucket_index = indexes.get_index_vec(to_bucket_field_name(&field_name))?;

            // Populate transpose_value_to_field_value_map
            for idx in cycle_epoch ..ordered_transpose_values.len() {
                let bucket_value = get_value_indirect(values, &field_to_bucket_index, idx)?;
                if bucket_value == &Value::Empty {
                    continue;
                }
                let bucket = bucket_spec.get_bucket(bucket_value);
                set_value_indirect(&mut output_values, &mut modified_columns, &bucket_index, idx, Value::Int(bucket as IntType))?;
                if  self.generate_bucket_range {
                    index_to_bucket_map.insert(idx, bucket);
                    min_values_for_bucket
                        .entry(bucket)
                        .and_modify(|min_value| if bucket_value < min_value { *min_value = bucket_value.clone(); })
                        .or_insert(bucket_value.clone());
                    max_values_for_bucket
                        .entry(bucket)
                        .and_modify(|max_value| if bucket_value > max_value { *max_value = bucket_value.clone(); })
                        .or_insert(bucket_value.clone());
                }
            }

            if  self.generate_bucket_range {
                for (idx, bucket) in index_to_bucket_map.iter() {
                    let bucket_range = format!(
                        "{} to {}",
                        min_values_for_bucket.get(&bucket).unwrap_or(&Value::Empty),
                        max_values_for_bucket.get(&bucket).unwrap_or(&Value::Empty)
                    );
                    set_value_indirect(&mut output_values, &mut modified_columns, &bucket_range_index, *idx, Value::String(bucket_range.clone().into()))?;
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

    #[test]
    fn test_commit_row_basic() {
        // Create a mock row with initial values
        let field_to_bucket = "price";

        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        // Instantiate BucketSpec with dummy range stops
        let bucket_spec = bucket_rng(field_to_bucket, vec![Value::Float(1.5), Value::Float(2.5)]);
        let bucket_data = BucketDataDiscrete {
            fields_to_bucket: vec![bucket_spec],
            generate_bucket_range: true,
        };

        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket, &"date1".into()), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date2".into()), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date3".into()), Value::Float(2.0)).unwrap();

        // Call commit_row and check the results
        let mut operator_row = BoxedOperatorRowTrait::new(row);
        let mut mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
        bucket_data.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date1".into())).unwrap(), Value::Int(0));
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_range_field_name(field_to_bucket), &"date1".into())).unwrap(), Value::String("1 to 1".into()));
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date2".into())).unwrap(), Value::Int(2));
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_range_field_name(field_to_bucket), &"date2".into())).unwrap(), Value::String("3 to 3".into()));
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date3".into())).unwrap(), Value::Int(1));
        assert_eq!(operator_row.get_value(&generate_column_name(&to_bucket_range_field_name(field_to_bucket), &"date3".into())).unwrap(), Value::String("2 to 2".into()));
    }

    fn create_mock_index(ordered_transpose_values: &Vec<Value>, bucket_data: &BucketDataDiscrete) -> MockIndexHolder {
        let mut mock_index = MockIndexHolder::new();
        for field in &bucket_data.fields_to_bucket {
            mock_index.register_index(field.field_to_bucket.to_string(), &ordered_transpose_values);
            mock_index.register_index(to_bucket_field_name(&field.field_to_bucket), &ordered_transpose_values);
            mock_index.register_index(to_bucket_range_field_name(&field.field_to_bucket), &ordered_transpose_values);
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

        // Instantiate BucketSpec with dummy range stops
        let bucket_spec = bucket_rng(field_to_bucket, vec![Value::Float(1.5), Value::Float(2.5)]);
        let bucket_data = BucketDataDiscrete {
            fields_to_bucket: vec![bucket_spec],
            generate_bucket_range: false,
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
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date1".into())).unwrap(), Value::Int(1));
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date2".into())).unwrap(), Value::Int(1));
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date3".into())).unwrap(), Value::Int(1));
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

        // Instantiate BucketSpec with dummy range stops
        let bucket_spec = bucket_rng(field_to_bucket, vec![Value::Float(1.5), Value::Float(2.5)]);
        let bucket_data = BucketDataDiscrete {
            fields_to_bucket: vec![bucket_spec],
            generate_bucket_range: false,
        };

        let mock_index = create_mock_index(&ordered_transpose_values, &bucket_data);
        let mut row = MockRow::new(&mock_index);
        row.set_value(&generate_column_name(field_to_bucket, &"date1".to_owned().into()), Value::Float(1.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date2".to_owned().into()), Value::Float(3.0)).unwrap();
        row.set_value(&generate_column_name(field_to_bucket, &"date3".to_owned().into()), Value::Float(2.0)).unwrap();

        let mut index = BoxedTransposeColumnIndexHolder::new(&mock_index);
        let mut row = BoxedOperatorRowTrait::new(row);
        bucket_data.commit_row(&mut row, &index, &ordered_transpose_values, 0).unwrap();

        // Check that the correct bucket values have been set
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date1".into())).unwrap(), Value::Int(0));
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date2".into())).unwrap(), Value::Int(2));
        assert_eq!(row.get_value(&generate_column_name(&to_bucket_field_name(field_to_bucket), &"date3".into())).unwrap(), Value::Int(1));
    }
}
