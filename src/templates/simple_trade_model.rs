use std::collections::HashMap;
use std::fmt::Display;
use crate::{context, get_string, IntType, TransposeColumnIndexHolder};
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};
use crate::context::{BoxedTransposeColumnIndex, BoxedTransposeColumnIndexHolder};
use crate::templates::utils::{get_value_indirect, get_value_indirect_from_row, set_value_indirect, set_value_indirect_if_some};

pub struct SimpleTradeModel {
    signal_field: String,
    price_value_field: String,
    conviction_field_name: String,
    instrument_field_name: String,
    initial_stop_loss_field_name: String,
    initial_take_profit_field_name: String,
    holding_period: IntType,
    re_entry_time: IntType
}


impl SimpleTradeModel {
    pub fn new(instrument_field_name: &str, signal_field_name: &str, price_value_field_name: &str, stop_loss_field_name:&str, take_profit_field_name:&str, conviction_field_name: &str, holding_period: IntType, re_entry_time: IntType) -> SimpleTradeModel {
        SimpleTradeModel {
            instrument_field_name: instrument_field_name.to_string(),
            signal_field: signal_field_name.to_string(),
            price_value_field: price_value_field_name.to_string(),
            conviction_field_name: conviction_field_name.to_string(),
            initial_stop_loss_field_name: stop_loss_field_name.to_string(),
            initial_take_profit_field_name: take_profit_field_name.to_string(),
            holding_period,
            re_entry_time
        }
    }
    }


impl CompiledTransposeCalculationTemplate for SimpleTradeModel {

    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            ("active_trade", ValueType::Boolean),
            ("initiation_date", ValueType::String),
            ("trade_id", ValueType::String),
            ("reason", ValueType::String),
            ("initiation_price", ValueType::Float),
            ("exit_price", ValueType::Float),
            ("stop_loss", ValueType::Float),
            ("trade_age", ValueType::Int),
            ("days_since_last_trade", ValueType::Int),
            ("delta", ValueType::Float),
            ("take_profit", ValueType::Float)
        ].iter().map(|(nm, val)|(nm.to_string(),*val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        vec![self.instrument_field_name.to_string(), self.signal_field.to_string(), self.price_value_field.to_string(),  self.conviction_field_name.to_string(),  self.initial_stop_loss_field_name.to_string(),  self.initial_take_profit_field_name.to_string()]
    }
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait,indexes: &BoxedTransposeColumnIndexHolder, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {
        let mut prev_trade_signal: Option<bool> = None;
        let mut active_trade: Option<bool> = None;
        let mut initiation_price: Option<FloatType> = None;
        let mut exit_price: Option<FloatType> = None;
        let mut initiation_date: Option<String> = None;
        let mut trade_id: Option<String> = None;
        let mut stop_loss: Option<FloatType> = None;
        let mut take_profit: Option<FloatType> = None;
        let mut delta: Option<FloatType> = None;
        let mut conviction: Option<FloatType> = None;
        let mut reason: Option<String> = None;
        let mut trade_age: Option<IntType> = None;
        let mut days_since_last_trade: Option<IntType> = None;
        
        let active_trade_index = indexes.get_index_vec("active_trade".to_owned())?;
        let trade_id_index = indexes.get_index_vec("trade_id".to_owned())?;
        let initiation_price_index = indexes.get_index_vec("initiation_price".to_owned())?;
        let trade_age_index = indexes.get_index_vec("trade_age".to_owned())?;
        let days_since_last_trade_index = indexes.get_index_vec("days_since_last_trade".to_owned())?;
        let initiation_date_index = indexes.get_index_vec("initiation_date".to_owned())?;
        let current_stop_loss_index = indexes.get_index_vec("stop_loss".to_owned())?;
        let current_take_profit_index = indexes.get_index_vec("take_profit".to_owned())?;
        let reason_index = indexes.get_index_vec("reason".to_owned())?;
        let delta_index = indexes.get_index_vec("delta".to_owned())?;
        let exit_price_index = indexes.get_index_vec("exit_price".to_owned())?;
        
        let instrument_index = indexes.get_index_vec(self.instrument_field_name.clone())?;
        let signal_index = indexes.get_index_vec(self.signal_field.clone())?;
        let price_index = indexes.get_index_vec(self.price_value_field.clone())?;
        let conviction_index = indexes.get_index_vec(self.conviction_field_name.clone())?;

        let initial_stop_loss_index = indexes.get_index_vec(self.initial_stop_loss_field_name.clone())?;
        let initial_take_profit_index = indexes.get_index_vec(self.initial_take_profit_field_name.clone())?;
        let mut instrument_name = None;
        
        let mut all_cols = signal_index.clone();
        all_cols.extend(price_index.clone());
        
        
        let mut all_values = row.get_values_for_columns(all_cols)?;
        let mut dirty_columns = vec![];

        if cycle_epoch > 0 {
            let transpose_index_before_epoch = cycle_epoch - 1;
            
            active_trade = get_value_indirect_from_row(row,&active_trade_index,transpose_index_before_epoch)?.as_boolean_or_none()?;
            trade_id = get_value_indirect_from_row(row,&trade_id_index,transpose_index_before_epoch)?.as_string_or_none()?;
            initiation_price =  get_value_indirect_from_row(row,&initiation_price_index,transpose_index_before_epoch)?.as_float_or_none()?;
            trade_age = get_value_indirect_from_row(row,&trade_age_index,transpose_index_before_epoch)?.as_int_or_none()?;
            initiation_date = get_value_indirect_from_row(row,&initiation_date_index,transpose_index_before_epoch)?.as_string_or_none()?;
            stop_loss = get_value_indirect_from_row(row,&current_stop_loss_index,transpose_index_before_epoch)?.as_float_or_none()?;
            take_profit = get_value_indirect_from_row(row,&current_take_profit_index,transpose_index_before_epoch)?.as_float_or_none()?;
            conviction = get_value_indirect_from_row(row,&conviction_index,transpose_index_before_epoch)?.as_float_or_none()?;
            reason = get_value_indirect_from_row(row,&reason_index,transpose_index_before_epoch)?.as_string_or_none()?;
            days_since_last_trade = get_value_indirect_from_row(row,&days_since_last_trade_index,transpose_index_before_epoch)?.as_int_or_none()?;            
        }

        if cycle_epoch > 1 {
            prev_trade_signal = get_value_indirect(&all_values, &signal_index, cycle_epoch - 2)?.as_boolean_or_none()?;
        }

        for i in cycle_epoch ..ordered_transpose_values.len() {
            if !instrument_name.is_some(){
                instrument_name = get_value_indirect_from_row(row,&instrument_index,i)?.as_string_or_none()?;
            }
            let transpose_value = &ordered_transpose_values[i];
            let current_signal = get_value_indirect(&all_values, &signal_index,i)?.as_boolean_or_none()?.unwrap_or_default();
            
            if let (Some(instrument_name),Some(current_close_value)) =
                (
                    instrument_name.as_ref(),
                    get_value_indirect(&all_values, &price_index,i)?.as_float_or_none()?
                )
            {
                //let current_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value))?.as_boolean_or_none()?.unwrap_or_default();
                let loop_active_trade = active_trade.is_some_and(|tv| tv);
                let mut loop_trade_closed = false;
                if loop_active_trade {
                    let loop_initiation_price = context(initiation_price, "Should have trade initiation price for active trade")?;
                    let loop_stop_loss = stop_loss;
                    let loop_take_profit =  take_profit;
                    let loop_conviction = conviction.as_ref().unwrap_or(&1f64);
                    trade_age = Some(context(trade_age, "Should have trade age for active trade")? + 1);
                    if loop_stop_loss.is_some_and(|sl| current_close_value <= sl) {
                        loop_trade_closed = true;
                        exit_price = Some(current_close_value);
                        let delta_value = (current_close_value - loop_initiation_price) * loop_conviction;
                        delta = Some(delta_value);
                        reason = Some(format!("{} {:.2} Closing trade. Current price ({:.2}) has fallen to or below stop loss {:.2} from entry price ({:.2}).", get_delta_label(delta_value), delta_value, current_close_value, loop_stop_loss.as_ref().unwrap(), loop_initiation_price));
                    } else if  loop_take_profit.is_some_and(|tp| current_close_value >= tp) {
                        loop_trade_closed = true;
                        exit_price = Some(current_close_value);
                        let delta_value = (current_close_value - loop_initiation_price) * loop_conviction;
                        delta = Some(delta_value);
                        reason = Some(format!("{} {:.2} Closing trade. Current price ({:.2}) has reached or exceeded take profit level {:.2} from entry price ({:.2}).",get_delta_label(delta_value),delta.unwrap(), current_close_value,loop_take_profit.as_ref().unwrap(), loop_initiation_price));
                    } else if trade_age.is_some_and(|age| age >= self.holding_period) {
                        loop_trade_closed = true;
                        exit_price = Some(current_close_value);
                        let delta_value = (current_close_value - loop_initiation_price) * loop_conviction;
                        delta = Some(delta_value);
                        reason = Some(format!("{} {:.2} Closing trade. Trade has reached holding period of ({}).",get_delta_label(delta_value),delta.unwrap(),trade_age.as_ref().unwrap()));
                    }
                } else {
                    if current_signal {
                        if !prev_trade_signal.unwrap_or_default() && (days_since_last_trade.is_none() || days_since_last_trade.unwrap() >= self.re_entry_time) {
                            initiation_price = Some(current_close_value);
                            initiation_date = Some(get_string(&transpose_value));
                            trade_id = Some(get_string(&transpose_value) + &instrument_name);
                            stop_loss = get_value_indirect_from_row(&row, &initial_stop_loss_index,i)?.as_float_or_none()?;
                            take_profit = get_value_indirect_from_row(&row,  &initial_take_profit_index,i)?.as_float_or_none()?;
                            active_trade = Some(true);
                            trade_age = Some(0);
                            days_since_last_trade = Some(0)
                        }
                    }else{
                        days_since_last_trade = Some(days_since_last_trade.unwrap_or_default() + 1);
                    }
                }

                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &active_trade_index, i, active_trade.map(Value::Boolean))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &reason_index, i, reason.map(|r|r.into()))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &initiation_price_index, i, initiation_price.map(Value::Float))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &initiation_date_index, i, initiation_date.clone().map(|r|r.into()))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &trade_id_index, i, trade_id.clone().map(|r|r.into()))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &delta_index, i, delta.clone().map(Value::Float))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &current_stop_loss_index, i, stop_loss.clone().map(Value::Float))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &exit_price_index, i, exit_price.clone().map(Value::Float))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &trade_age_index, i, trade_age.clone().map(Value::Int))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &current_take_profit_index, i, take_profit.clone().map(Value::Float))?;
                set_value_indirect_if_some(&mut all_values, &mut  dirty_columns, &days_since_last_trade_index, i, days_since_last_trade.clone().map(Value::Int))?;
                prev_trade_signal = Some(current_signal);
                reason = None;
                delta = None;
                exit_price = None;
                if loop_trade_closed {
                    active_trade = Some(false);
                    days_since_last_trade = Some(0);
                    initiation_price = None;
                    initiation_date = None;
                    trade_id = None;
                    trade_age = None;
                    stop_loss = None;
                    take_profit = None;
                }
            }
        }
        row.set_values_for_columns(dirty_columns, all_values)?;
        
        Ok(())
    }
}

fn get_delta_label(delta: FloatType) -> String {
    if delta == 0.0 {
        ""
    }
    else if delta > 0.0 {
        "Won"
    } else {
        "Lost"
    }.to_string()
}


mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::templates::test_utils::{MockIndexHolder, MockRow};

    #[test]
    fn test_commit_row_initial_trade() -> Result<(), Error> {
        // Set up initial values for a new trade
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        let model = SimpleTradeModel::new(
            "instrument",
            "signal",
            "price",
            "stop_loss_initial",
            "take_profit_initial",
            "conviction",
            10, // holding_period
            5   // re_entry_time
        );

        let mock_index = create_mock_index(&ordered_transpose_values, &model);
        let mut row = MockRow::new(&mock_index);
        
        row.set_value_for_transpose_index("instrument",0, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("signal",0, Value::Boolean(true))?;
        row.set_value_for_transpose_index("price",0, Value::Float(100.0))?;
        row.set_value_for_transpose_index("stop_loss",0, Value::Float(95.0))?;
        row.set_value_for_transpose_index("take_profit",0, Value::Float(110.0))?;
        row.set_value_for_transpose_index("conviction",0, Value::Float(1.0))?;

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();
        }

        // Verify that a new trade has been initiated correctly
        assert_eq!(row.get_value_for_transpose_index("active_trade",0)?, Value::Boolean(true));
        assert_eq!(row.get_value_for_transpose_index("initiation_date",0)?, "date1".to_owned().into());
        assert_eq!(row.get_value_for_transpose_index("trade_id",0)?, "date1instrument1".to_owned().into());
        assert_eq!(row.get_value_for_transpose_index("initiation_price",0)?, Value::Float(100.0));
        assert_eq!(row.get_value_for_transpose_index("stop_loss",0)?, Value::Float(95.0));
        assert_eq!(row.get_value_for_transpose_index("take_profit",0)?, Value::Float(110.0));
        assert_eq!(row.get_value_for_transpose_index("trade_age",0)?, Value::Int(0));
        assert_eq!(row.get_value_for_transpose_index("reason",0)?, Value::Empty);
        assert_eq!(row.get_value_for_transpose_index("delta",0)?, Value::Empty);
        assert_eq!(row.get_value_for_transpose_index("exit_price",0)?, Value::Empty);
        
        Ok(())
    }

    #[test]
    fn test_commit_row_trade_closure_on_stop_loss() -> Result<(), Error> {
        // Set up a scenario where the stop loss is hit, and the trade should be closed
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        let model = SimpleTradeModel::new(
            "instrument",
            "signal",
            "price",
            "initial_stop_loss",
            "initial_take_profit",
            "conviction",
            10, // holding_period
            5   // re_entry_time
        );

        let mock_index = create_mock_index(&ordered_transpose_values, &model);
        let mut row = MockRow::new(&mock_index);

        // Set initial values to initiate the trade
        row.set_value_for_transpose_index("instrument",0, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("signal",0, Value::Boolean(true))?;
        row.set_value_for_transpose_index("price",0, Value::Float(100.0))?;
        row.set_value_for_transpose_index("initial_stop_loss",0, Value::Float(95.0))?;
        row.set_value_for_transpose_index("take_profit",0, Value::Float(110.0))?;
        row.set_value_for_transpose_index("conviction",0, Value::Float(1.0))?;
        // Set initial values to initiate the trade

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();
        }

        row.set_value_for_transpose_index("instrument",1, "instrument1".to_owned().into())?;
        // Simulate a price drop to hit the stop loss
        row.set_value_for_transpose_index("price",1, Value::Float(94.0))?;
        row.set_value_for_transpose_index("instrument",2, "instrument1".to_owned().into())?;
        // Simulate a price drop to hit the stop loss
        row.set_value_for_transpose_index("price",2, Value::Float(94.0))?;

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 1).unwrap();
        }

        // Verify that the trade has been closed due to stop loss
        assert_eq!(row.get_value_for_transpose_index("active_trade",0)?, Value::Boolean(true));
        assert_eq!(row.get_value_for_transpose_index("active_trade",1)?, Value::Boolean(true));
        assert_eq!(row.get_value_for_transpose_index("active_trade",2)?, Value::Boolean(false));
        assert_eq!(row.get_value_for_transpose_index("exit_price",1)?, Value::Float(94.0));
        assert!(matches!(row.get_value_for_transpose_index("reason",1)?, Value::String(ref reason) if reason.ref_into_owned().contains("stop loss")));

        Ok(())
    }

    #[test]
    fn test_commit_row_trade_closure_on_take_profit() -> Result<(), Error> {
        // Set up a scenario where the take profit is hit, and the trade should be closed
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
        ];

        let model = SimpleTradeModel::new(
            "instrument",
            "signal",
            "price",
            "initial_stop_loss",
            "initial_take_profit",
            "conviction",
            10, // holding_period
            5   // re_entry_time
        );

        let mock_index = create_mock_index(&ordered_transpose_values, &model);
        let mut row = MockRow::new(&mock_index);

        // Set initial values to initiate the trade
        row.set_value_for_transpose_index("instrument",0, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("signal",0, Value::Boolean(true))?;
        row.set_value_for_transpose_index("price",0, Value::Float(100.0))?;
        row.set_value_for_transpose_index("stop_loss",0, Value::Float(95.0))?;
        row.set_value_for_transpose_index("take_profit",0, Value::Float(110.0))?;
        row.set_value_for_transpose_index("conviction",0, Value::Float(1.0))?;

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();
        }

        // Simulate a price rise to hit the take profit
        row.set_value_for_transpose_index("instrument",1, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("price",1, Value::Float(111.0))?;
        // Simulate a price rise to hit the take profit
        row.set_value_for_transpose_index("instrument",2, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("price",2, Value::Float(111.0))?;

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 1).unwrap();
        }

        // Verify that the trade has been closed due to take profit
        assert_eq!(row.get_value_for_transpose_index("active_trade",1)?, Value::Boolean(true));
        assert_eq!(row.get_value_for_transpose_index("active_trade",2)?, Value::Boolean(false));
        assert_eq!(row.get_value_for_transpose_index("exit_price",1)?, Value::Float(111.0));
        assert!(matches!(row.get_value_for_transpose_index("reason",1)?, Value::String(ref reason) if reason.ref_into_owned().contains("take profit")));

        Ok(())
    }

    #[test]
    fn test_commit_row_trade_holding_period_expiry() -> Result<(), Error> {
        // Set up a scenario where the trade is closed due to holding period expiry
        let ordered_transpose_values = vec![
            "date1".to_string().into(),
            "date2".to_string().into(),
            "date3".to_string().into(),
            "date4".to_string().into(),
        ];

        let model = SimpleTradeModel::new(
            "instrument",
            "signal",
            "price",
            "initial_stop_loss",
            "initial_take_profit",
            "conviction",
            2, // holding_period set to 2
            5  // re_entry_time
        );

        let mock_index = create_mock_index(&ordered_transpose_values, &model);
        let mut row = MockRow::new(&mock_index);

        // Set initial values to initiate the trade
        row.set_value_for_transpose_index("instrument",0, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("signal",0, Value::Boolean(true))?;
        row.set_value_for_transpose_index("price",0, Value::Float(100.0))?;
        row.set_value_for_transpose_index("initial_stop_loss",0, Value::Float(95.0))?;
        row.set_value_for_transpose_index("initial_take_profit",0, Value::Float(110.0))?;
        row.set_value_for_transpose_index("conviction",0, Value::Float(1.0))?;

        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 0).unwrap();
        }

        // Simulate the passing of time beyond the holding period
        row.set_value_for_transpose_index("instrument",1, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("price",1, Value::Float(102.0))?;
        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 1).unwrap();
        }

        row.set_value_for_transpose_index("instrument",2, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("price",2, Value::Float(103.0))?;

        row.set_value_for_transpose_index("instrument",3, "instrument1".to_owned().into())?;
        row.set_value_for_transpose_index("price",3, Value::Float(103.0))?;
        {
            let mut operator_row = BoxedOperatorRowTrait::new(&mut row);
            let mock_index_holder = BoxedTransposeColumnIndexHolder::new(&mock_index);
            model.commit_row(&mut operator_row, &mock_index_holder, &ordered_transpose_values, 2).unwrap();
        }

        // Verify that the trade has been closed due to holding period expiry
        assert_eq!(row.get_value_for_transpose_index("active_trade",2)?, Value::Boolean(true));
        assert_eq!(row.get_value_for_transpose_index("exit_price",2)?, Value::Float(103.0));
        assert!(matches!(row.get_value_for_transpose_index("reason",2)?, Value::String(ref reason) if reason.ref_into_owned().contains("holding period")));
        assert_eq!(row.get_value_for_transpose_index("active_trade",3)?, Value::Boolean(false));

        Ok(())
    }


    fn create_mock_index(ordered_transpose_values: &Vec<Value>, model: &SimpleTradeModel) -> MockIndexHolder {
        let mut mock_index = MockIndexHolder::new();
        let fields = vec![
            &model.instrument_field_name,
            &model.signal_field,
            &model.price_value_field,
            &model.conviction_field_name,
            &model.initial_stop_loss_field_name,
            &model.initial_take_profit_field_name,
            "active_trade",
            "initiation_price",
            "days_since_last_trade",
            "exit_price",
            "initiation_date",
            "trade_id",
            "stop_loss",
            "take_profit",
            "trade_age",
            "reason",
            "delta"
        ];

        for field in fields {
            mock_index.register_index(field.to_string(), &ordered_transpose_values);
        }
        mock_index
    }
}



