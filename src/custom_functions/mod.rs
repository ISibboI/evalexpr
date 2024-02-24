pub mod triangular_moving_average;
pub mod is_null;
pub mod simple_moving_average;
pub mod simple_cumulative_sum;


// Re-export the functions to the root of the crate
pub use triangular_moving_average::triangular_moving_average;
pub use is_null::is_null;
pub use simple_moving_average::simple_moving_average;
pub use simple_cumulative_sum::simple_cumulative_sum;
