pub mod adaptive_stop_loss_trade_model;
pub mod compiled_transpose_calcuation_template;
mod channel_monitoring_template;
mod test_utils;

use std::fmt::Display;
// Re-export the functions to the root of the crate
pub use channel_monitoring_template::*;
pub use adaptive_stop_loss_trade_model::*;
pub use compiled_transpose_calcuation_template::*;
use crate::Value;

