pub mod triangular_moving_average;
pub mod expression_functions;
pub mod simple_moving_average;
pub mod simple_cumulative_sum;
pub mod back;
mod calc_conviction;
mod compiled_transpose_calcuation_template;


// Re-export the functions to the root of the crate
pub use triangular_moving_average::triangular_moving_average;
pub use expression_functions::*;
pub use back::*;
pub use simple_moving_average::simple_moving_average;
pub use simple_cumulative_sum::simple_cumulative_sum;
