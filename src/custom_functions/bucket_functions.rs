use crate::Value;
use crate::Error;
use crate::IntType;
use crate::Error::CustomError;
macro_rules! generate_bucket_functions {
    ($($name:ident($($stop:ident),+)),+) => {
        $(
            paste::paste! {
                pub fn $name<T, $([<T $stop>]),+>(value_to_bucket: T, $($stop: [<T $stop>]),+) -> Result<Value, Error>
                where
                    T: std::convert::TryInto<Value>,
                    <T as std::convert::TryInto<Value>>::Error: core::fmt::Debug,
                    $([<T $stop>]: std::convert::TryInto<Value>, <[<T $stop>] as std::convert::TryInto<Value>>::Error: core::fmt::Debug),+
                {
                    let value_to_bucket: Value = value_to_bucket.try_into().map_err(|_| Error::CustomError("Failed to convert value_to_bucket".to_string()))?;
                    let stops = vec![$($stop.try_into().map_err(|_| Error::CustomError("Failed to convert stop".to_string()))?),+];

                    // Ensure stops are in ascending order
                    for i in 0..stops.len() - 1 {
                        if stops[i] > stops[i + 1] {
                            return Err(Error::CustomError("Stops must be in ascending order".to_string()));
                        }
                    }

                    // Determine which bucket the value belongs to
                    for (i, stop) in stops.iter().enumerate() {
                        if value_to_bucket <= *stop {
                            return Ok(Value::Int(i as IntType));
                        }
                    }

                    Ok(Value::Int(stops.len() as IntType))
                }

                pub fn [<$name _desc>] <T, $([<T $stop>]),+>(value_to_bucket: T, $($stop: [<T $stop>]),+) -> Result<Value, Error>
                where
                    T: std::convert::TryInto<Value>,
                    <T as std::convert::TryInto<Value>>::Error: core::fmt::Debug,
                    $([<T $stop>]: std::convert::TryInto<Value>, <[<T $stop>] as std::convert::TryInto<Value>>::Error: core::fmt::Debug),+
                {
                    let value_to_bucket: Value = value_to_bucket.try_into().map_err(|_| Error::CustomError("Failed to convert value_to_bucket".to_string()))?;
                    let stops = vec![$($stop.try_into().map_err(|_| Error::CustomError("Failed to convert stop".to_string()))?),+];

                    // Ensure stops are in ascending order
                    for i in 0..stops.len() - 1 {
                        if stops[i] > stops[i + 1] {
                            return Err(Error::CustomError("Stops must be in ascending order".to_string()));
                        }
                    }

                    // Determine which bucket the value belongs to and return the description
                    for (i, stop) in stops.iter().enumerate() {
                        if value_to_bucket <= *stop {
                            if i == 0 {
                                return Ok(Value::String(format!(".<= {}", stop).into()));
                            } else {
                                return Ok(Value::String(format!("{} - {}", stops[i - 1], stop).into()));
                            }
                        }
                    }

                    Ok(Value::String(format!("> {}", stops.last().unwrap()).into()))
                }
            }
        )+
    };
}

// Generate bucket and bucket description functions with 2 to 5 stops
generate_bucket_functions! {
    bucket_2(stop_1, stop_2),
    bucket_3(stop_1, stop_2, stop_3),
    bucket_4(stop_1, stop_2, stop_3, stop_4),
    bucket_5(stop_1, stop_2, stop_3, stop_4, stop_5)
}



#[cfg(test)]
mod tests {
    use crate::Value;
    use super::*;

    // Assuming generate_bucket_functions! macro has already generated these functions:
    // bucket_2, bucket_2_desc, bucket_3, bucket_3_desc, etc.

    #[test]
    fn test_bucket_2() {
        let stop_1 = Value::Int(10);
        let stop_2 = Value::Int(20);

        assert_eq!(bucket_2(Value::Int(5), stop_1.clone(), stop_2.clone()).unwrap(), Value::Int(0));
        assert_eq!(bucket_2(Value::Int(15), stop_1.clone(), stop_2.clone()).unwrap(), Value::Int(1));
        assert_eq!(bucket_2(Value::Int(25), stop_1.clone(), stop_2.clone()).unwrap(), Value::Int(2));
    }

    #[test]
    fn test_bucket_2_desc() {
        let stop_1 = Value::Int(10);
        let stop_2 = Value::Int(20);

        assert_eq!(bucket_2_desc(Value::Int(5), stop_1.clone(), stop_2.clone()).unwrap(), Value::String("<= 10".into()));
        assert_eq!(bucket_2_desc(Value::Int(15), stop_1.clone(), stop_2.clone()).unwrap(), Value::String("10 - 20".into()));
        assert_eq!(bucket_2_desc(Value::Int(25), stop_1.clone(), stop_2.clone()).unwrap(), Value::String("> 20".into()));
    }

    #[test]
    fn test_bucket_3() {
        let stop_1 = Value::Int(10);
        let stop_2 = Value::Int(20);
        let stop_3 = Value::Int(30);

        assert_eq!(bucket_3(Value::Int(5), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::Int(0));
        assert_eq!(bucket_3(Value::Int(15), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::Int(1));
        assert_eq!(bucket_3(Value::Int(25), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::Int(2));
        assert_eq!(bucket_3(Value::Int(35), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::Int(3));
    }

    #[test]
    fn test_bucket_3_desc() {
        let stop_1 = Value::Int(10);
        let stop_2 = Value::Int(20);
        let stop_3 = Value::Int(30);

        assert_eq!(bucket_3_desc(Value::Int(5), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::String("<= 10".into()));
        assert_eq!(bucket_3_desc(Value::Int(15), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::String("10 - 20".into()));
        assert_eq!(bucket_3_desc(Value::Int(25), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::String("20 - 30".into()));
        assert_eq!(bucket_3_desc(Value::Int(35), stop_1.clone(), stop_2.clone(), stop_3.clone()).unwrap(), Value::String("> 30".into()));
    }


    #[test]
    fn test_bucket_5_desc_with_float_stops() {
        let stop_1 = Value::Float(10.5);
        let stop_2 = Value::Float(20.5);
        let stop_3 = Value::Float(30.5);
        let stop_4 = Value::Float(40.5);
        let stop_5 = Value::Float(50.5);

        assert_eq!(
            bucket_5_desc(Value::Int(5), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("<= 10.5".into())
        );
        assert_eq!(
            bucket_5_desc(Value::Int(15), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("10.5 - 20.5".into())
        );
        assert_eq!(
            bucket_5_desc(Value::Int(25), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("20.5 - 30.5".into())
        );
        assert_eq!(
            bucket_5_desc(Value::Int(35), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("30.5 - 40.5".into())
        );
        assert_eq!(
            bucket_5_desc(Value::Int(45), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("40.5 - 50.5".into())
        );
        assert_eq!(
            bucket_5_desc(Value::Int(55), stop_1.clone(), stop_2.clone(), stop_3.clone(), stop_4.clone(), stop_5.clone()).unwrap(),
            Value::String("> 50.5".into())
        );
    }

    #[test]
    fn test_invalid_stops() {
        let stop_1 = Value::Int(20);
        let stop_2 = Value::Int(10);

        let result = bucket_2(stop_1.clone(), stop_1.clone(), stop_2.clone());
        assert!(result.is_err());

        let desc_result = bucket_2_desc(stop_1.clone(), stop_1.clone(), stop_2.clone());
        assert!(desc_result.is_err());
    }
}
