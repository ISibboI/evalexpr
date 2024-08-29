use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, IntType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder, TransposeColumnIndex, TransposeColumnIndexHolder};
use crate::Error::CustomError;
use crate::templates::test_utils::MockIndexHolder;
use crate::templates::utils::{get_value_indirect, set_value_indirect};


pub  struct CorrelationAnalysis{
    independent_variables: Vec<String>,
    dependent_variable: String,
}

fn to_coeficient(field_name: &str) -> String {
    format!("{}coefficient", field_name)
}
fn to_pvalue(field_name: &str) -> String {
    format!("{}pvalue", field_name)
}
fn to_rsquared(field_name: &str) -> String {
    format!("{}rsquared", field_name)
}
fn to_adjusted_rsquared(field_name: &str) -> String {
    format!("{}adjustedrsquared", field_name)
}

impl CorrelationAnalysis {
    pub fn new(dependent_variable: &str, independent_variables: Vec<&str>) -> CorrelationAnalysis {
        CorrelationAnalysis {
            independent_variables: independent_variables.iter().map(|fld| fld.to_string()).collect(),
            dependent_variable: dependent_variable.to_string(),
        }
    }
}
impl CompiledTransposeCalculationTemplate for CorrelationAnalysis {
    fn schema(&self) -> HashMap<String, ValueType> {
        let mut result = vec![];
        for fld in self.independent_variables.iter() {
            result.push((to_coeficient(fld), ValueType::Float));
            result.push((to_pvalue(fld), ValueType::Float));
            result.push((to_rsquared(fld), ValueType::Float));
            result.push((to_adjusted_rsquared(fld), ValueType::Float));
        }
        result.iter().map(|(nm, val)|(nm.to_string(),*val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        let mut vec1 = self.independent_variables.clone();
        vec1.extend(vec![self.dependent_variable.clone()]);
        vec1
    }
    fn commit_row(
        &self,
        row: &mut BoxedOperatorRowTrait,
        indexes: &BoxedTransposeColumnIndexHolder,
        ordered_transpose_values: &[Value],
        cycle_epoch: usize,
    ) -> Result<(), Error> {

        let values = &row.get_values()?;
        let mut output_values = vec![Value::Empty; values.len()];
        let mut modified_columns = vec![];
        row.set_values_for_columns(modified_columns, output_values)?;
        Ok(())
    }
}
