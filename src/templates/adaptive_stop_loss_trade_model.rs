use std::collections::HashMap;
use std::fmt::Display;
use crate::{context, get_string, IntType};
use crate::{BoxedOperatorRowTrait, CompiledTransposeCalculationTemplate, Error, FloatType, generate_column_name, OperatorRowTrait, Value, ValueType};


pub struct AdaptiveStopLossTradeModel {
    signal_field: String,
    close_value_field: String,
    ask_value_field: String,
    trading_range_field_name: String,
    instrument_field_name: String,
    stop_loss_threshold: FloatType,
    take_profit_threshold: FloatType,
    break_even_threshold: FloatType
}


impl AdaptiveStopLossTradeModel {
    pub fn new(instrument_field_name: &str,signal_field_name: &str, close_value_field_name: &str,  ask_value_field_name: &str, trading_range_field_name: &str, stop_loss_threshold: FloatType, take_profit_threshold: FloatType, break_even_threshold: FloatType) -> AdaptiveStopLossTradeModel {
        AdaptiveStopLossTradeModel {
            instrument_field_name: instrument_field_name.to_string(),
            signal_field: signal_field_name.to_string(),
            close_value_field: close_value_field_name.to_string(),
            ask_value_field: ask_value_field_name.to_string(),
            trading_range_field_name: trading_range_field_name.to_string(),
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
            ("trade_id", ValueType::String),
            ("reason", ValueType::String),
            ("initiation_price", ValueType::Float),
            ("exit_price", ValueType::Float),
            ("stop_loss", ValueType::Float),
            ("trade_age", ValueType::Int),
            ("delta", ValueType::Float),
            ("take_profit", ValueType::Float),
            ("avg_daily_range", ValueType::Float),
            ("break_even", ValueType::Float)
        ].iter().map(|(nm, val)|(nm.to_string(),*val)).collect()
    }
    fn dependencies(&self) -> Vec<String> {
        vec![self.instrument_field_name.to_string(), self.signal_field.to_string(), self.close_value_field.to_string(), self.trading_range_field_name.to_string(), self.ask_value_field .to_string()]
    }
    fn commit_row(&self, row: &mut BoxedOperatorRowTrait, ordered_transpose_values: &[Value], cycle_epoch: usize) -> Result<(), Error> {
        let mut prev_trade_signal: Option<bool> = None;
        let mut active_trade: Option<bool> = None;
        let mut initiation_price: Option<FloatType> = None;
        let mut exit_price: Option<FloatType> = None;
        let mut initiation_date: Option<String> = None;
        let mut trade_id: Option<String> = None;
        let mut stop_loss: Option<FloatType> = None;
        let mut take_profit: Option<FloatType> = None;
        //let mut break_even: Option<FloatType> = None;
        let mut delta: Option<FloatType> = None;
        let mut reason: Option<String> = None;
        let mut trade_age: Option<IntType> = None;
        let MIN_TRADE_DURATION = 5;

        if cycle_epoch > 0 {
            let transpose_value_before_epoch = &ordered_transpose_values[cycle_epoch - 1];
            active_trade = row.get_value(&generate_column_name("active_trade", transpose_value_before_epoch))?.as_boolean_or_none()?;
            trade_id = row.get_value(&generate_column_name("trade_id", transpose_value_before_epoch))?.as_string_or_none()?;
            initiation_price = row.get_value(&generate_column_name("initiation_price", transpose_value_before_epoch))?.as_float_or_none()?;
            trade_age = row.get_value(&generate_column_name("trade_age", transpose_value_before_epoch))?.as_int_or_none()?;
            initiation_date = row.get_value(&generate_column_name("initiation_date", transpose_value_before_epoch))?.as_string_or_none()?;
            stop_loss = row.get_value(&generate_column_name("stop_loss", transpose_value_before_epoch))?.as_float_or_none()?;
            take_profit = row.get_value(&generate_column_name("take_profit", transpose_value_before_epoch))?.as_float_or_none()?;
            //break_even = row.get_value(&generate_column_name("break_even", transpose_value_before_epoch))?.as_float_or_none()?;
            reason = row.get_value(&generate_column_name("reason", transpose_value_before_epoch))?.as_string_or_none()?;
        }

        if cycle_epoch > 1 {
            let transpose_value_before_epoch = &ordered_transpose_values[cycle_epoch - 2];
            prev_trade_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value_before_epoch))?.as_boolean_or_none()?;
        }

        for i in cycle_epoch ..ordered_transpose_values.len() {
            let transpose_value = &ordered_transpose_values[i];
            if let (Some(instrument_name),Some(current_close_value),Some(current_ask_value),Some(trading_range)) =
                (
                    row.get_value(&generate_column_name(&self.instrument_field_name, transpose_value))?.as_string_or_none()?,
                    row.get_value(&generate_column_name(&self.close_value_field, transpose_value))?.as_float_or_none()?,
                    row.get_value(&generate_column_name(&self.ask_value_field, transpose_value))?.as_float_or_none()?,
                    row.get_value(&generate_column_name(&self.trading_range_field_name, transpose_value))?.as_float_or_none()?
                )
            {
                let current_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value))?.as_boolean_or_none()?.unwrap_or_default();
                let loop_active_trade = active_trade.is_some_and(|tv| tv);
                let mut loop_trade_closed = false;
                if loop_active_trade {
                    let loop_initiation_price = context(initiation_price, "Should have trade initiation price for active trade")?;
                    let loop_stop_loss = context(stop_loss, "Should have stop loss for active trade")?;
                    let loop_take_profit = context(take_profit, "Should have take profit active trade")?;
                    //let loop_break_even = context(break_even, "Should break even on active trade")?;
                    let next_stop_loss_step = loop_stop_loss + ((trading_range * self.break_even_threshold) * 2f64);
                    trade_age = Some(context(trade_age, "Should have trade age for active trade")? + 1);
                    if current_close_value <= loop_stop_loss {
                        if trade_age.unwrap_or_default() >= MIN_TRADE_DURATION {
                            loop_trade_closed = true;
                            exit_price = Some(current_close_value);
                            delta = Some(current_close_value - loop_initiation_price);
                            reason = Some(format!("Lost {} Closing trade. Current price ({}) has fallen to or below stop loss {}({}) from entry price ({}).", delta.unwrap(), current_close_value, loop_stop_loss, self.stop_loss_threshold, loop_initiation_price));
                        }
                    } else if current_close_value >= loop_take_profit {
                        loop_trade_closed = true;
                        exit_price = Some(current_close_value);
                        delta = Some(current_close_value - loop_initiation_price);
                        reason = Some(format!("Won {} Closing trade. Current price ({}) has reached or exceeded take profit level {} from entry price ({}).",delta.unwrap(), current_close_value,loop_take_profit, loop_initiation_price));
                    } else if current_close_value > next_stop_loss_step {
                        //stop_loss = Some(loop_stop_loss + (trading_range * self.break_even_threshold));
                    }
                } else {
                    let loop_trade_signal = row.get_value(&generate_column_name(&self.signal_field, transpose_value))?.as_boolean_or_none()?.unwrap_or_default();
                    if loop_trade_signal {
                        if !prev_trade_signal.unwrap_or_default() {
                            initiation_price = Some(current_ask_value);
                            initiation_date = Some(get_string(&transpose_value));
                            trade_id = Some(get_string(&transpose_value) + &instrument_name);
                            stop_loss = Some(current_close_value - (trading_range *self.stop_loss_threshold));
                            take_profit = Some(current_close_value + (trading_range *self.take_profit_threshold));
                            //break_even = Some(current_close_value + (trading_range *self.break_even_threshold));
                            active_trade = Some(true);
                            trade_age = Some(0);
                        }
                    }
                }

                row.set_value(&generate_column_name("active_trade", transpose_value), Value::Boolean(active_trade.unwrap_or_default()))?;
                row.set_value(&generate_column_name("reason", transpose_value), reason.clone().map(|rs| Value::String(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("initiation_price", transpose_value), initiation_price.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("initiation_date", transpose_value), initiation_date.clone().map(|rs| Value::String(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("trade_id", transpose_value), trade_id.clone().map(|rs| Value::String(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("delta", transpose_value), delta.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("exit_price", transpose_value), exit_price.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("stop_loss", transpose_value), stop_loss.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("take_profit", transpose_value), take_profit.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("trade_age", transpose_value), trade_age.clone().map(|rs| Value::Int(rs)).unwrap_or(Value::Empty))?;
                //row.set_value(&generate_column_name("break_even", transpose_value), break_even.clone().map(|rs| Value::Float(rs)).unwrap_or(Value::Empty))?;
                row.set_value(&generate_column_name("avg_daily_range", transpose_value), Value::Float(trading_range.clone()))?;
                prev_trade_signal = Some(current_signal);
                reason = None;
                delta = None;
                exit_price = None;
                if loop_trade_closed {
                    active_trade = Some(false);
                    initiation_price = None;
                    initiation_date = None;
                    trade_id = None;
                    trade_age = None;
                    stop_loss = None;
                    take_profit = None;
                   // break_even = None;
                }
            }
        }
        Ok(())
    }
}


