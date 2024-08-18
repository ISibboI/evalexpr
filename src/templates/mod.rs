pub mod adaptive_stop_loss_trade_model;
pub mod ordered_float;
pub mod bucket_data_template;
pub mod compiled_transpose_calcuation_template;
pub  mod channel_monitoring_template;
pub(crate) mod test_utils;

use std::fmt::Display;
// Re-export the functions to the root of the crate
pub use channel_monitoring_template::*;
pub use adaptive_stop_loss_trade_model::*;
pub use bucket_data_template::*;
pub use compiled_transpose_calcuation_template::*;
use crate::Value;

