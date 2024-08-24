pub mod adaptive_stop_loss_trade_model;
pub mod ordered_float;
pub mod bucket_data_template;
pub mod compiled_transpose_calculation_template;
pub  mod channel_monitoring_template;
pub  mod simple_trade_model;
pub(crate) mod test_utils;
pub  mod rolling_zscore;
mod utils;

use std::fmt::Display;
// Re-export the functions to the root of the crate
pub use channel_monitoring_template::*;
pub use adaptive_stop_loss_trade_model::*;
pub use simple_trade_model::*;
pub use bucket_data_template::*;
pub use rolling_zscore::*;
pub use compiled_transpose_calculation_template::*;
use crate::Value;

