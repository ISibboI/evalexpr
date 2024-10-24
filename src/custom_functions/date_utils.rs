use std::convert::TryInto;
use std::fmt::Debug;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use workdays::WorkCalendar;
use crate::{Error, Value};
use crate::Error::CustomError;

lazy_static! {
    static ref BUSINESS_DAY_CALENDAR: WorkCalendar = WorkCalendar::new();
}
pub fn business_days_between<TL: TryInto<Value>, TR: TryInto<Value>>(start_date: TL, end_date: TR) -> Result<Value, Error>
where
    <TL as TryInto<Value>>::Error: Debug,
    <TR as TryInto<Value>>::Error: Debug,
{
    // Convert inputs to strings and then to NaiveDate
    let mut start_date_str = start_date.try_into().map_err(|err| CustomError(format!("{err:?}")))?.as_string()?;
    let mut end_date_str = end_date.try_into().map_err(|err| CustomError(format!("{err:?}")))?.as_string()?;

    // Trim the date strings if they are longer than 10 characters
    if start_date_str.len() > 10 {
        start_date_str = start_date_str[..10].to_string();
    }
    if end_date_str.len() > 10 {
        end_date_str = end_date_str[..10].to_string();
    }

    // Parse the strings into NaiveDate
    let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
        .map_err(|err| CustomError(format!("Invalid start date format: {err:?}")))?;
    let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
        .map_err(|err| CustomError(format!("Invalid end date format: {err:?}")))?;

    // Ensure end_date is after or equal to start_date
    if end_date < start_date {
        return Err(CustomError("End date must be after start date".into()));
    }
    let days = BUSINESS_DAY_CALENDAR.work_days_between(start_date, end_date);
    // Return the result as an integer value
    Ok(Value::Int(days))
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_business_days_between_valid_range() {
        let start_date = "2024-10-20"; // Sunday
        let end_date = "2024-10-30";   // Wednesday
        let result = business_days_between(start_date, end_date).unwrap();
        assert_eq!(result, Value::Int(8)); // Excludes weekends
    }

    #[test]
    fn test_business_days_between_same_day() {
        let start_date = "2024-10-23"; // Wednesday
        let end_date = "2024-10-23";   // Same Wednesday
        let result = business_days_between(start_date, end_date).unwrap();
        assert_eq!(result, Value::Int(1)); // 1 business day
    }

    #[test]
    fn test_business_days_between_weekend_range() {
        let start_date = "2024-10-19"; // Saturday
        let end_date = "2024-10-22";   // Tuesday
        let result = business_days_between(start_date, end_date).unwrap();
        assert_eq!(result, Value::Int(2)); // Excludes the weekend
    }

    #[test]
    fn test_business_days_between_invalid_date() {
        let start_date = "invalid-date";
        let end_date = "2024-10-22";
        let result = business_days_between(start_date, end_date);
        assert!(result.is_err()); // Should return an error for invalid date
    }

    #[test]
    fn test_business_days_between_end_before_start() {
        let start_date = "2024-10-25"; // Friday
        let end_date = "2024-10-23";   // Wednesday
        let result = business_days_between(start_date, end_date);
        assert!(result.is_err()); // Should return an error when end_date is before start_date
    }
}
