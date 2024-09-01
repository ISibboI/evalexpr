use std::collections::HashMap;
use std::convert::TryInto;

use crate::{Error, Value};
use crate::Error::CustomError;
#[cfg(feature = "serde_json_support")]
use serde_json::json;
//#[cfg(feature = "serde_json_support")]

#[cfg(feature = "serde_json_support")]
pub fn create_report_reference_nodes_from_operator_uri<TL: TryInto<Value>,TR: TryInto<Value>>(uri: TL, output_property_name: TR) ->  Result<Value, Error>
where <TL as TryInto<crate::Value>>::Error: std::fmt::Display,<TR as TryInto<crate::Value>>::Error: std::fmt::Display
{
    let operator_uri = uri.try_into().map_err(|err| CustomError(format!("{err}")))?.as_string()?;
    let output_property_name = output_property_name.try_into().map_err(|err| CustomError(format!("{err}")))?.as_string()?;
    let params = extract_parameters(&operator_uri);
    let report_key = extract_report_key(&operator_uri);
    let result = json!([{
        "name": "report_reference_node",
        "operatorType": "report_reference",
        "nodeType": "Persistent",
        "parameterValues": params,
        "outputOperatorName": output_property_name,
        "reportKey": report_key,
    }]);
    Ok(Value::String(serde_json::to_string(&result).map_err(|err| CustomError(format!("{err}")))?.into()))
}
pub fn extract_report_key_from_operator_uri<TL: TryInto<Value>>(uri: TL) ->  Result<Value, Error>
where <TL as TryInto<crate::Value>>::Error: std::fmt::Display
{
    let operator_uri = uri.try_into().map_err(|err| CustomError(format!("{}",err)))?.as_string()?;
    Ok(Value::String(operator_uri.into()))
}

fn extract_parameters(input_string: &str) -> HashMap<String, String> {
    // Find the index of the word 'params' in the string
    if let Some(params_index) = input_string.find("params") {
        // Strip everything before 'params'
        let stripped_string = &input_string[params_index + "params".len()..];
        // Split the string by 'bbb' to get the individual property pairs
        let pairs = stripped_string.split("bbb");

        // Create an empty HashMap to store the parameter values
        let mut params_dict = HashMap::new();

        // Iterate over each pair
        for pair in pairs {
            // Split each pair by 'yyy' to separate the key and the value
            let parts: Vec<&str> = pair.split("yyy").collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                // Add them to the dictionary if both key and value exist
                params_dict.insert(key, value);
            }
        }

        return params_dict;
    }

    // Return an empty HashMap if 'params' is not found
    HashMap::new()
}


fn extract_report_key(input_string: &str) -> String {
    // Find the index of the forward slash '/'
    if let Some(slash_index) = input_string.find('/') {
        // Extract the part before the slash
        let prefix = &input_string[..slash_index];

        // Remove the starting 'xx' if it exists
        if prefix.starts_with("xx") {
            return prefix[2..].to_string();
        } else {
            return prefix.to_string();
        }
    }

    // If no slash is found, handle the prefix normally
    let prefix = input_string;
    if prefix.starts_with("xx") {
        prefix[2..].to_string()
    } else {
        prefix.to_string()
    }
}