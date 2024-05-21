use std::collections::HashMap;
use std::fmt::Display;
use crate::{context, get_string};
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};


pub struct AdaptiveStopLossTradeModel {
    signal_field: String,
    value_field: String,
    stop_loss_threshold: FloatType,
    take_profit_threshold: FloatType,
    break_even_threshold: FloatType
}


impl AdaptiveStopLossTradeModel {
    pub fn new(signal_field_name: &str, value_field_name: &str, stop_loss_threshold: FloatType, take_profit_threshold: FloatType, break_even_threshold: FloatType) -> AdaptiveStopLossTradeModel {
        AdaptiveStopLossTradeModel {
            signal_field: signal_field_name.to_string(),
            value_field: value_field_name.to_string(),
            stop_loss_threshold,
            take_profit_threshold,
            break_even_threshold
        }
    }
    }


impl CompiledTransposeCalculationTemplate for AdaptiveStopLossTradeModel {
    fn schema(&self) -> HashMap<String, ValueType> {
        vec![
            ("active_trade", ValueType::Boolean),
            ("initiation_date", ValueType::String),
            ("reason", ValueType::String),
            ("initiation_price", ValueType::Float),
            ("stop_loss", ValueType::Float),
            ("delta", ValueType::Float),
            ("take_profit", ValueType::Float),
            ("break_even", ValueType::Float)
        ].iter().map(|(nm, val)|(nm.to_string(),*val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        vec![self.signal_field.to_string(), self.value_field.to_string()]
    }
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {


        let mut prev_trade_signal: Option<bool> = None;


        let mut active_trade: Option<bool> = None;
        let mut initiation_price: Option<FloatType> = None;
        let mut initiation_date: Option<String> = None;
        let mut stop_loss: Option<FloatType> = None;
        let mut take_profit: Option<FloatType> = None;
        let mut break_even: Option<FloatType> = None;
        let mut delta: Option<FloatType> = None;
        let mut reason: Option<String> = None;

        if cycle_epoch > 0 {
            let transpose_value_before_epoch = &ordered_transpose_values[cycle_epoch - 1];
            active_trade = row.get_value(&generate_column_name("active_trade", transpose_value_before_epoch))?.as_boolean_or_none()?;
            initiation_price = row.get_value(&generate_column_name("initiation_price", transpose_value_before_epoch))?.as_float_or_none()?;
            initiation_date = row.get_value(&generate_column_name("initiation_date", transpose_value_before_epoch))?.as_string_or_none()?;
            stop_loss = row.get_value(&generate_column_name("stop_loss", transpose_value_before_epoch))?.as_float_or_none()?;
            take_profit = row.get_value(&generate_column_name("take_profit", transpose_value_before_epoch))?.as_float_or_none()?;
            break_even = row.get_value(&generate_column_name("break_even", transpose_value_before_epoch))?.as_float_or_none()?;
            reason = row.get_value(&generate_column_name("reason", transpose_value_before_epoch))?.as_string_or_none()?;
        }

        if cycle_epoch > 1 {
            let transpose_value_before_epoch = &ordered_transpose_values[cycle_epoch - 2];
            prev_trade_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value_before_epoch))?.as_boolean_or_none()?;
        }

        for i in cycle_epoch ..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];
            if let Some(current_close_value) =
                row.get_value(&generate_column_name(&self.value_field, transpose_value))?.as_float_or_none()?
            {
                let current_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value))?.as_boolean_or_none()?.unwrap_or_default();
                let loop_active_trade = active_trade.is_some_and(|tv| tv);
                let mut loop_trade_closed = false;
                if loop_active_trade {
                    let loop_initiation_price = context(initiation_price, "Should have trade initiation price for active trade")?;
                    let loop_stop_loss = context(stop_loss, "Should have stop loss for active trade")?;
                    let loop_take_profit = context(take_profit, "Should have take profit active trade")?;
                    let loop_break_even = context(break_even, "Should break even on active trade")?;

                    let delta_val = current_close_value - loop_initiation_price;
                    if current_close_value <= loop_stop_loss {
                        loop_trade_closed = true;
                        delta = Some(delta_val);
                        reason = Some(format!("Lost {} Closing trade. Current price ({}) has fallen to or below stop loss {} from entry price ({}).",delta_val, current_close_value,loop_stop_loss, loop_initiation_price));
                    } else if current_close_value >= loop_take_profit {
                        loop_trade_closed = true;
                        delta = Some(current_close_value - loop_initiation_price);
                        reason = Some(format!("Won {} Closing trade. Current price ({}) has reached or exceeded take profit level {} from entry price ({}).",delta_val, current_close_value,loop_take_profit, loop_initiation_price));
                    } else if current_close_value > loop_break_even {
                        stop_loss = Some(loop_initiation_price + self.break_even_threshold);
                    }
                } else {
                    let loop_trade_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value))?.as_boolean_or_none()?.unwrap_or_default();
                    if loop_trade_signal {
                        if !prev_trade_signal.unwrap_or_default() {
                            initiation_price = Some(current_close_value);
                            initiation_date = Some(get_string(&transpose_value));
                            stop_loss = Some(current_close_value - self.stop_loss_threshold);
                            take_profit = Some(current_close_value + self.take_profit_threshold);
                            break_even = Some(current_close_value + self.break_even_threshold);
                            active_trade = Some(true);
                        }
                    }
                }

                row.set_value(&generate_column_name("active_trade", transpose_value), Value::Boolean(active_trade.unwrap_or_default()))?;
                row.set_value(&generate_column_name("reason", transpose_value), reason.clone().map(|rs| Value::String(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("initiation_price", transpose_value), initiation_price.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("initiation_date", transpose_value), initiation_date.clone().map(|rs| Value::String(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("delta", transpose_value), delta.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("stop_loss", transpose_value), stop_loss.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("take_profit", transpose_value), take_profit.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("break_even", transpose_value), break_even.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                prev_trade_signal = Some(current_signal);
                reason = None;
                delta = None;
                if loop_trade_closed {
                    active_trade = Some(false);
                    initiation_price = None;
                    initiation_date = None;
                    stop_loss = None;
                    take_profit = None;
                    break_even = None;
                }
            }
        }
        Ok(())
    }
}


