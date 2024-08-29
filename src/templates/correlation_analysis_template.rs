use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
// use ndarray::{array, stack, Array, Axis};
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType, IntType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder, TransposeColumnIndex, TransposeColumnIndexHolder};
use crate::Error::CustomError;
use crate::templates::test_utils::MockIndexHolder;
use crate::templates::utils::{get_value_indirect, set_value_indirect};
use ndarray::prelude::*;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

pub struct CorrelationAnalysis {
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
        result.iter().map(|(nm, val)| (nm.to_string(), *val)).collect()
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
        //let mut output_values = vec![Value::Empty; values.len()];
        //let mut modified_columns = vec![];

        let mut y = vec![0f64; ordered_transpose_values.len()];
        let mut x_data = vec![vec![]; self.independent_variables.len()];
        let dependent_variables = indexes.get_index_vec(self.dependent_variable.clone())?;

        let mut previous_value = 0f64;
        for index in &dependent_variables {
            let result = get_value_indirect(values, &dependent_variables, *index)?.as_float_or_none()?;
            if let Some(val) = result {
                previous_value = val;
                y.push(val);
            } else {
                y.push(previous_value);
            }
        }

        for dep in &self.independent_variables {
            let dependent_variables = indexes.get_index_vec(dep.clone())?;
            let mut previous_value = 0f64;
            let mut result = vec![];
            for index in &dependent_variables {
                let val = get_value_indirect(values, &dependent_variables, *index)?.as_float_or_none()?;
                if let Some(val) = val {
                    previous_value = val;
                    result.push(val);
                } else {
                    result.push(previous_value);
                }
            }
            x_data.push(result);
        }


        let x: Array2<f64> = Array::from_shape_vec(
            (y.len(), x_data.len() + 1), // +1 for the intercept column
            x_data.into_iter().flat_map(|v| v.into_iter()).collect(),
        ).map_err(|err| CustomError(format!("Shape error found {err}")))?;



        let y = Array::from_vec(y);

        // // Perform the regression: beta = (X'X)^(-1)X'y
        // let xtx = x.t().dot(&x); // X'X
        // let xty = x.t().dot(&y); // X'y
        // let beta = xtx.solve_into(xty).map_err(|err| CustomError(format!("Regression failed {err}")))?;
        // 
        // // Calculate the predicted values
        // let y_pred = x.dot(&beta);
        // 
        // // Calculate R-squared
        // let ss_total = y.mapv(|yi| (yi - y.mean().unwrap()).powi(2)).sum();
        // let ss_residual = y.iter().zip(y_pred.iter()).map(|(yi, y_pred_i)| (yi - y_pred_i).powi(2)).sum::<f64>();
        // let r_squared = 1.0 - (ss_residual / ss_total);
        // 
        // // Output the coefficients (betas)
        // println!("Regression coefficients (betas): {:?}", beta);
        // 
        // // Output the R-squared value
        // println!("R-squared: {}", r_squared);
        
        Ok(())
    }
}

mod tests {
    use super::*;
    use crate::templates::test_utils::{MockIndexHolder, MockRow};
    use ndarray::array;

    fn create_mock_row<'a>(mock_index: &'a MockIndexHolder) -> MockRow<'a> {
        // Mock row with some values
        let mut row = MockRow::new(mock_index);

        row.set_value("independent_var1__date1", Value::Float(1.0)).unwrap();
        row.set_value("independent_var1__date2", Value::Float(2.0)).unwrap();
        row.set_value("independent_var1__date3", Value::Float(3.0)).unwrap();
        row.set_value("dependent_var__date1", Value::Float(4.0)).unwrap();
        row.set_value("dependent_var__date2", Value::Float(5.0)).unwrap();
        row.set_value("dependent_var__date3", Value::Float(6.0)).unwrap();

        row
    }

    #[test]
    fn test_correlation_analysis_basic() {
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        let independent_vars = vec!["independent_var1"];
        let dependent_var = "dependent_var";

        let analysis = CorrelationAnalysis::new(dependent_var, independent_vars);
        let mut mock_index = MockIndexHolder::new();

        let mut row = create_mock_row(&mock_index);
        let mock_index = create_mock_index(&ordered_transpose_values);
        let mut operator_row = BoxedOperatorRowTrait::new(row);
        let mut mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);

        analysis
            .commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0)
            .unwrap();

        // Expected outputs
        assert!(operator_row.get_value(&to_coeficient("independent_var1")).is_ok());
        assert!(operator_row.get_value(&to_pvalue("independent_var1")).is_ok());
        assert!(operator_row.get_value(&to_rsquared("independent_var1")).is_ok());
        assert!(operator_row.get_value(&to_adjusted_rsquared("independent_var1")).is_ok());
    }

    fn create_mock_index(ordered_transpose_values: &[Value]) -> MockIndexHolder {
        let mut mock_index = MockIndexHolder::new();
        mock_index.register_index("independent_var1".to_string(), ordered_transpose_values);
        mock_index.register_index("dependent_var".to_string(), ordered_transpose_values);
        mock_index
    }
}

