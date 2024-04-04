// use crate::{BoxedThinTraitContext, Error, FloatType, IntType, TupleType, Value};
//
//
// struct CursorCommon{
//     position: Option<usize>,
//     length: usize,
//
// }
// trait StatefulCursor<'a>{
//     fn common(&self) -> &'a CursorCommon;
//     fn common_mut(&mut self) -> &'a mut CursorCommon;
//
//     fn initialize(&mut self, context: Box<dyn Exp>) -> Result<(), Error>{
//         self.common_mut().length = length;
//     }
//     fn next(&mut self) -> Option<usize>{
//         let new_position = self.common().position.unwrap_or(0) + 1;
//         if new_position < self.common().length {
//             self.common_mut().position = Some(new_position);
//             Some(new_position)
//         } else {
//             None
//         }
//     }
//
//     fn commit(&mut self) -> Result<(), Error>{
//         loop {
//             if let Some(position) = self.next() {
//                 continue;
//             }else{
//                 break;
//             }
//         }
//         self.common_mut().position = None;
//         Ok(())
//     }
//     fn read(&self, field: &str, index: usize) -> Result<Value, Error>;
//     fn write(&self, field: &str, value: &Value,index: usize) -> Result<(), Error>;
// }
//
// struct TradeInitiationCursor<'a> {
//     position: Option<usize>,
//     prices_since_inception: &'a [Value],
//     tmac_lower_band: &'a [Value],
// }
//
// impl StatefulCursor for TradeInitiationCursor{
//     fn next(&self) -> Option<usize> {
//         let new_position = self.position.unwrap_or(0) + 1;
//         if new_position < self.prices_since_inception.len() {
//             Some(new_position)
//         } else {
//             None
//         }
//     }
//
//     fn read(&self, field: &str, index: usize) -> Result<Value, Error> {
//         todo!()
//     }
//
//     fn write(&self, field: &str, value: &Value, index: usize) -> Result<(), Error> {
//         todo!()
//     }
// }
//
//
// impl ActiveTrade<'_> {
//     fn new(prices_since_inception: &[f64]) -> ActiveTrade {
//         ActiveTrade {
//             prices_since_inception,
//         }
//     }
//
//     fn should_close(&self) -> Result<(bool, String), Error> {
//         let current_price = self.prices_since_inception.first().ok_or(Error::CustomError("Not enough values to retrieve the current price of active trade".to_owned()))?;
//         let entry_price = self.prices_since_inception.last().ok_or(Error::CustomError("Not enough values to retrieve the entry price of active trade".to_owned()))?;
//         let mut stop_loss = *entry_price - 0.0002; // 2 pips below entry
//         let take_profit = *entry_price + 0.0003; // 3 pips above entry
//
//         if *current_price <= stop_loss {
//             // Close trade if price falls 2 pips below entry
//             return Ok((true, format!("Closing trade. Current price ({}) has fallen to or below stop loss from entry price ({}).", current_price, entry_price)));
//         } else if *current_price >= take_profit {
//             // Take profit if price rises 3 pips above entry
//             return Ok((true, format!("Closing trade. Current price ({}) has reached or exceeded take profit level from entry price ({}).", current_price, entry_price)));
//         } else if *current_price >= *entry_price + 0.0001 {
//             // Adjust stop loss to break-even if price rises by at least 1 pip
//             stop_loss = *entry_price;
//             if *current_price <= stop_loss {
//                 // Close trade if after adjustment, current price falls to stop loss
//                 return Ok((true, format!("Closing trade. After adjusting stop loss to break-even, current price ({}) has reached stop loss level from entry price ({}).", current_price, entry_price)));
//             }
//         }
//
//         // If none of the conditions to close the trade are met
//         Ok((false, Default::default()))
//     }
// }
//
//
// fn generate_trades<'a>(prices: &'a[f64], signals: &'a[bool]) -> Result<Vec<ActiveTrade<'a>>,Error> {
//     let mut trades: Vec<ActiveTrade> = Vec::new();
//     let mut active_trade: Option<ActiveTrade> = None;
//     let mut trade_start_index: usize = 0;
//
//     for (i, &signal) in signals.iter().enumerate() {
//         if signal && active_trade.is_none() && i < prices.len() {
//             // Start a new trade
//             trade_start_index = i;
//             active_trade = Some(ActiveTrade::new(&prices[i..]));
//         }
//
//         if let Some(trade) = &active_trade {
//             let close_result = trade.should_close()?;
//             if close_result.0 || i == prices.len() - 1 {
//                 // Close the active trade and push to trades list
//                 trades.push(ActiveTrade::new(&prices[trade_start_index..=i]));
//                 active_trade = None; // Ensure no overlapping trades
//             }
//         }
//     }
//
//     Ok(trades)
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_trade_close_conditions() {
//         let prices = [1.1050, 1.1048, 1.1051, 1.1054];
//         let trade = ActiveTrade::new(&prices);
//
//         let (should_close, reason) = trade.should_close().unwrap();
//         assert!(should_close, "Trade should close due to reaching take profit.");
//         assert_eq!(reason, "Closing trade. Current price (1.1054) has reached or exceeded take profit level from entry price (1.1050).");
//     }
//
//     #[test]
//     fn test_generate_trades_with_signals() {
//         let prices = [1.1050, 1.1048, 1.1051, 1.1052, 1.1053, 1.1050, 1.1048, 1.1054];
//         let signals = [true, false, false, true, false, true, false, false];
//         let trades = generate_trades(&prices, &signals).unwrap();
//
//         // Expected to create trades based on the signals, but they should not overlap
//         assert_eq!(trades.len(), 3, "Expected three trades based on the given signals.");
//
//         // Ensure trades are initialized with the correct prices
//         assert_eq!(*trades[0].prices_since_inception.last().unwrap(), 1.1050, "First trade entry price should be 1.1050.");
//         assert_eq!(*trades[1].prices_since_inception.last().unwrap(), 1.1052, "Second trade entry price should be 1.1052.");
//         assert_eq!(*trades[2].prices_since_inception.last().unwrap(), 1.1050, "Third trade entry price should be 1.1050.");
//     }
//
//     #[test]
//     fn test_trade_non_closure_on_insufficient_movement() {
//         let prices = [1.1050, 1.1051]; // Not enough movement to trigger any of the close conditions
//         let trade = ActiveTrade::new(&prices);
//
//         let (should_close, reason) = trade.should_close().unwrap();
//         assert!(!should_close, "Trade should not close due to insufficient price movement.");
//         assert_eq!(reason, "", "There should be no reason for closing the trade.");
//     }
//
//     #[test]
//     fn test_error_on_empty_price_array() {
//         let prices = [];
//         let trade = ActiveTrade::new(&prices);
//
//         let result = trade.should_close();
//         assert!(result.is_err(), "Should return error due to empty price array.");
//         if let Err(Error::CustomError(msg)) = result {
//             assert_eq!(msg, "Not enough values to retrieve the current price of active trade");
//         } else {
//             panic!("Expected CustomError for empty price array.");
//         }
//     }
//
//     #[test]
//     fn    generate_trades_non_overlapping() {
//         let prices = [1.1050, 1.1048, 1.1052, 1.1054, 1.1051, 1.1053, 1.1050];
//         let signals = [true, false, true, false, false, true, false];
//         let trades = generate_trades(&prices, &signals).expect("Failed to generate trades");
//
//         assert_eq!(trades.len(), 3, "Should generate 3 non-overlapping trades based on signals");
//
//         // Verify that each trade has correctly identified start and potential close conditions
//         // Simplifying assumption: each trade closes on the next signal or the end of the array
//         // Note: In real scenarios, you'd check for specific conditions met for closing each trade
//         let expected_entry_prices = [1.1050, 1.1052, 1.1053];
//         for (i, trade) in trades.iter().enumerate() {
//             assert_eq!(*trade.prices_since_inception.last().unwrap(), expected_entry_prices[i], "Entry price does not match expected");
//         }
//     }
//
//     #[test]
//     fn generate_trades_with_no_signals() {
//         let prices = [1.1050, 1.1051, 1.1052, 1.1053];
//         let signals = [false, false, false, false]; // No signals to generate trades
//         let trades = generate_trades(&prices, &signals).expect("Failed to generate trades");
//
//         assert!(trades.is_empty(), "Should generate no trades without any signals");
//     }
//
//     #[test]
//     fn generate_trades_with_all_signals() {
//         let prices = [1.1050, 1.1049, 1.1051, 1.1052];
//         let signals = [true, true, true, true]; // Every point is a signal
//         let trades = generate_trades(&prices, &signals).expect("Failed to generate trades");
//
//         // Expecting to generate trades only for the first signal, as subsequent signals are ignored until the current trade is closed
//         // Assuming that each trade must close before another begins
//         assert_eq!(trades.len(), 1, "Should generate only one trade despite signals due to non-overlapping rule");
//     }
// }
