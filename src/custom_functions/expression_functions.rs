use crate::{Error, FloatType, Value};
use chrono::{NaiveDateTime,Timelike,Utc, DateTime, Duration, Datelike, TimeZone};

pub fn is_null<T: Into<Value>>(value: T) ->  Result<Value, Error>  {
    Ok(match value.into() {
        Value::Empty => Value::Int(0),
        v => v,
    })
}

pub fn abs<T: Into<Value>>(value: T) -> Result<Value, Error> {
    match value.into() {
        Value::Float(fl) => { Ok(Value::Float(fl.abs())) }
        Value::Int(nn) => { Ok(Value::Int(nn.abs())) }
        Value::Empty => {Ok(Value::Empty)}
        _ => Err(Error::InvalidArgumentType),
    }
}
pub fn safe_divide<TL: Into<Value>,TR: Into<Value>>(left: TL, right: TR) -> Result<Value, Error> {
    match (left.into(), right.into()) {
        (Value::Float(left), Value::Float(right)) => {
            if right == 0.0 {
                Ok(Value::Empty)
            } else {
                Ok(Value::Float(left / right))
            }
        }
        (Value::Int(left), Value::Int(right)) => {
            if right == 0 {
                Ok(Value::Empty)
            } else {
                Ok(Value::Int(left / right))
            }
        }
        (Value::Float(left), Value::Int(right)) => {
            if right == 0 {
                Ok(Value::Empty)
            } else {
                Ok(Value::Float(left / right as FloatType))
            }
        }
        (Value::Int(left), Value::Float(right)) => {
            if right == 0.0 {
                Ok(Value::Empty)
            } else {
                Ok(Value::Float(left as FloatType / right))
            }
        }
        (_, Value::Empty) => {
            Ok(Value::Empty)
        },
        (Value::Empty,_) => {
            Ok(Value::Empty)
        }

        _ => Err(Error::InvalidArgumentType),
    }
}

pub fn starts_with(message: &Value, prefix: &Value) ->  Result<Value, Error>  {
    if let (Value::String(message), Value::String(prefix)) = (message, prefix) {
        if message.starts_with(prefix) {
            return Ok(Value::Boolean(true));
        }
    }
    return Ok(Value::Boolean(false));
}

fn round_datetime_to_precision(datetime: DateTime<Utc>, precision: &str) -> Result<DateTime<Utc>, crate::Error> {
    Ok(match precision {
        "m1" => datetime.date().and_hms(datetime.hour(), datetime.minute(), 0),
        "m5" => datetime.date().and_hms(datetime.hour(), (datetime.minute() / 5) * 5, 0),
        "m15" => datetime.date().and_hms(datetime.hour(), (datetime.minute() / 15) * 15, 0),
        "m30" => datetime.date().and_hms(datetime.hour(), (datetime.minute() / 30) * 30, 0),
        "h1" => datetime.date().and_hms(datetime.hour(), 0, 0),
        "h4" => datetime.date().and_hms((datetime.hour() / 4) * 4, 0, 0),
        "d1" => datetime.date().and_hms(0, 0, 0),
        "1w" => (datetime - Duration::days(datetime.date().weekday().num_days_from_sunday() as i64)).date().and_hms(0, 0, 0),
        "1M" => datetime.date().with_day(1).unwrap().and_hms(0, 0, 0),
        val => {
            return Err(Error::CustomError(format!("Precision {val} is not recognised")));
        } // If the precision is not recognized, return the original datetime
    })
}

pub fn round_date_to_precision(string: &Value, precision: &Value) -> Result<Value, crate::Error> {
    if let (Value::String(string), Value::String(precision)) = (string, precision) {
        // Extract the date-time part from the input string
        let parts: Vec<&str> = string.split('_').collect();
        let datetime_str = parts.last().ok_or_else(|| Error::InvalidInputString)?;

        let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y.%m.%d %H:%M:%S")
            .map_err(|_| Error::InvalidDateFormat)?;
        let datetime = Utc.from_utc_datetime(&naive_datetime);
        let rounded_datetime = round_datetime_to_precision(datetime, &precision.to_lowercase())?;
        let result = format!("{}_{}", parts.iter().take(parts.len() - 1).map(|prt|prt.to_string()).collect::<Vec<String>>().join("_"), rounded_datetime.format("%Y.%m.%d %H:%M:%S").to_string());

        Ok(Value::String(result))
    } else {
        // If arguments are not strings, return an error
        Err(Error::InvalidArgumentType)
    }
}

pub fn max<TL: Into<Value>,TR: Into<Value>>(value1: TL, value2: TR) ->  Result<Value, Error>  {
    let x = value1.into();
    let x1 = value2.into();
    Ok(if x > x1 {
        x
    } else {
        x1
    })
}

pub fn min<TL: Into<Value>,TR: Into<Value>>(value1: TL, value2: TR) -> Result<Value, Error>  {
    let x = value1.into();
    let x1 = value2.into();
    Ok(if x < x1 {
        x
    } else {
        x1
    })
}


mod test{

    use super::*;
    #[test]
    fn test_round_date_to_m1() {
        let input = (
            Value::String("BTCUSD_2024.02.13 10:05:23".into()),
            Value::String("m1".into())
        );
        let expected = Utc.ymd(2024, 2, 13).and_hms(10, 5, 0).format("%Y.%m.%d %H:%M:%S").to_string();
        let result = round_date_to_precision(&input.0, &input.1).unwrap();
        assert_eq!(result, Value::String(format!("BTCUSD_{}", expected)));
    }

    #[test]
    fn test_round_date_to_h1() {
        let input = (
            Value::String("BTCUSD_2024.02.13 10:05:23".into()),
            Value::String("h1".into())
        );
        let expected = Utc.ymd(2024, 2, 13).and_hms(10, 0, 0).format("%Y.%m.%d %H:%M:%S").to_string();
        let result = round_date_to_precision(&input.0, &input.1).unwrap();
        assert_eq!(result, Value::String(format!("BTCUSD_{}", expected)));
    }

    #[test]
    fn test_round_date_to_1w() {
        let input = (
            Value::String("BTCUSD_2024.02.13 10:05:23".into()),
            Value::String("1w".into())
        );
        // Assuming 2024-02-13 is a Wednesday, rounding to the start of the week (Sunday)
        let expected = Utc.ymd(2024, 2, 11).and_hms(0, 0, 0).format("%Y.%m.%d %H:%M:%S").to_string();
        let result = round_date_to_precision(&input.0, &input.1).unwrap();
        assert_eq!(result, Value::String(format!("BTCUSD_{}", expected)));
    }

    #[test]
    fn test_invalid_date_format() {
        let input = (
            Value::String("BTCUSD_ThisIsNotADate".into()),
            Value::String("m1".into())
        );
        let result = round_date_to_precision(&input.0, &input.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_precision() {
        let input = (
            Value::String("BTCUSD_2024.02.13 10:05:00".into()),
            Value::String("m60".into())
        );
        let result = round_date_to_precision(&input.0, &input.1);
        assert!(result.is_err(), "Expected an error for invalid precision");
    }
}
