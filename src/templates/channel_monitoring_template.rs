use std::collections::HashMap;
use std::fmt::Display;

use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder};

pub struct ChannelMonitor {
    lower_band_field_name: String,
    upper_band_field_name: String,
    value_field_name: String,
    output_field_name: String,
    within_bounds_field_name: String,
    left_above_field_name: String,
    left_below_field_name: String,
    re_entered_above_field_name: String,
    re_entered_below_field_name: String
}

impl ChannelMonitor {
    pub fn new(lower_band_field_name: &str, upper_band_field_name: &str, value_field_name: &str, output_field_name: &str) -> ChannelMonitor {

        let within_bounds_field_name = format!("{}_within_bounds", output_field_name);
        let left_above_field_name = format!("{}_left_above", output_field_name);
        let left_below_field_name = format!("{}_left_below", output_field_name);
        let re_entered_above_field_name = format!("{}_re_entered_above", output_field_name);
        let re_entered_below_field_name = format!("{}_re_entered_below", output_field_name);

        ChannelMonitor {
            lower_band_field_name : lower_band_field_name.to_owned(),
            upper_band_field_name : upper_band_field_name.to_owned(),
            value_field_name : value_field_name.to_owned(),
            within_bounds_field_name,
            left_above_field_name,
            left_below_field_name,
            re_entered_above_field_name,
            re_entered_below_field_name,
            output_field_name : output_field_name.to_owned()
        }
    }
}

impl CompiledTransposeCalculationTemplate for ChannelMonitor {

    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            (self.within_bounds_field_name.clone(), ValueType::Boolean),
            (self.within_bounds_field_name.clone(), ValueType::Boolean),
            (self.left_above_field_name.clone(), ValueType::Boolean),
            (self.left_below_field_name.clone(), ValueType::Boolean),
            (self.re_entered_above_field_name.clone(), ValueType::Boolean),
            (self.re_entered_below_field_name.clone(), ValueType::Boolean),
        ].iter().map(|(nm, val)| (nm.to_string(), *val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        vec![
            self.lower_band_field_name.clone(),
            self.upper_band_field_name.clone(),
            self.value_field_name.clone()
        ]
    }
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait ,indexes: &BoxedTransposeColumnIndexHolder, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {

        let mut within_bounds: Option<bool> = None;
        let mut left_above: Option<bool> = None;
        let mut left_below: Option<bool> = None;

        if cycle_epoch > 0 {
            let transpose_value_before_epoch = &ordered_transpose_values[cycle_epoch - 1];
            within_bounds = row.get_value(&generate_column_name(&self.within_bounds_field_name, transpose_value_before_epoch))?.as_boolean_or_none()?;
            left_above = row.get_value(&generate_column_name(&self.left_above_field_name, transpose_value_before_epoch))?.as_boolean_or_none()?;
            left_below = row.get_value(&generate_column_name(&self.left_below_field_name, transpose_value_before_epoch))?.as_boolean_or_none()?;
        }

        for i in cycle_epoch ..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];

            let loop_lower_band = row.get_value(&generate_column_name(&self.lower_band_field_name, transpose_value))?.as_float_or_none()?;
            let loop_upper_band = row.get_value(&generate_column_name(&self.upper_band_field_name, transpose_value))?.as_float_or_none()?;
            let loop_value = row.get_value(&generate_column_name(&self.value_field_name, transpose_value))?.as_float_or_none()?;

            if let (Some(lower_band), Some(upper_band), Some(value)) = (loop_lower_band, loop_upper_band, loop_value) {
                let loop_within_bounds = value >= lower_band && value <= upper_band;
                let loop_left_above = value > upper_band;
                let loop_left_below = value < lower_band;
                let loop_re_entered_above = loop_within_bounds && left_above.unwrap_or_default();
                let loop_re_entered_below = loop_within_bounds && left_below.unwrap_or_default();

                row.set_value(&generate_column_name(&self.within_bounds_field_name, transpose_value), Value::Boolean(loop_within_bounds))?;
                row.set_value(&generate_column_name(&self.left_above_field_name, transpose_value), Value::Boolean(loop_left_above))?;
                row.set_value(&generate_column_name(&self.left_below_field_name, transpose_value), Value::Boolean(loop_left_below))?;
                row.set_value(&generate_column_name(&self.re_entered_above_field_name, transpose_value), Value::Boolean(loop_re_entered_above))?;
                row.set_value(&generate_column_name(&self.re_entered_below_field_name, transpose_value), Value::Boolean(loop_re_entered_below))?;

                within_bounds = Some(loop_within_bounds);
                left_above = Some(loop_left_above);
                left_below = Some(loop_left_below);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod commit_row_tests {
    use super::*;
    use std::collections::HashMap;
    use crate::templates::test_utils::{MockIndexHolder, MockRow};

    #[test]
    fn test_commit_row_value_below_lower_band() -> Result<(), Error> {
        let monitor = ChannelMonitor::new(
            "lower_band",
            "upper_band",
            "value",
            "output",
        );

        let holder = MockIndexHolder::new();
        let mut row = MockRow::new(&holder);
        let cycle_epoch = 0;
        let transpose_value = Value::String("02_01_2024".to_owned());
        let ordered_transpose_values = vec![transpose_value.clone()];

        row.insert_value(generate_column_name("lower_band", &transpose_value), Value::Float(10.0));
        row.insert_value(generate_column_name("upper_band", &transpose_value), Value::Float(20.0));
        row.insert_value(generate_column_name("value", &transpose_value), Value::Float(5.0));

        let mut row_trait = BoxedOperatorRowTrait::new(row);
        let mut index_hodlder_trait = BoxedTransposeColumnIndexHolder::new(&holder);
        monitor.commit_row(&mut row_trait,&index_hodlder_trait, &ordered_transpose_values, cycle_epoch)?;

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

        let mock_index_column_holder = MockIndexHolder::new();
        let mut row = MockRow::new(&mock_index_column_holder);
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
        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        monitor.commit_row(&mut row_trait,&index_holder_trait, &ordered_transpose_values, 0)?;

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

        let mut mock_index_column_holder = MockIndexHolder::new();
        let mut row = MockRow::new(&mock_index_column_holder);
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
        let mut index_holder_trait = BoxedTransposeColumnIndexHolder::new(&mock_index_column_holder);
        monitor.commit_row(&mut row_trait,&index_holder_trait, &ordered_transpose_values, 0)?;

        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_below", &current_transpose_value))?, Value::Boolean(false));
        assert_eq!(row_trait.get_value(&generate_column_name("output_re_entered_above", &current_transpose_value))?, Value::Boolean(true));

        Ok(())
    }

}