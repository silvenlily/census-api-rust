use std::str::FromStr;

use serde_json::Value;

use crate::utils::CensusError;

pub fn parse_string(key: &str, json: &Value) -> Result<String, CensusError> {
    if !json[key].is_string() {
        return Err(CensusError {
            err_msg: "Malformed Service Message, could not parse field: '".to_string()
                + key
                + "' to string.",
            parent_err: None,
        });
    }
    return Ok(json[key].as_str().unwrap().to_string());
}

pub fn try_parse_to<T: FromStr>(key: &str, json: &Value) -> Result<T, CensusError> {
    if !json[key].is_string() {
        return Err(CensusError {
            err_msg: "Malformed Service Message, could not find field: '".to_string()
                + key
                + "'",
            parent_err: None,
        });
    };

    let v_str = json[key].as_str().unwrap();

    let v: Result<T, <T as FromStr>::Err> = v_str.parse::<T>();

    match v {
        Ok(res) => {
            return Ok(res);
        }
        Err(_) => {
            return Err(CensusError {
                err_msg: "Malformed Service Message, could not parse field: '".to_string()
                    + key
                    + "'",
                parent_err: None,
            });
        }
    }
}

pub fn parse_character_id(json: &Value) -> Result<String, CensusError> {
    return parse_string("character_id", json);
}

pub fn parse_timestamp(json: &Value) -> Result<String, CensusError> {
    return parse_string("timestamp", json);
}

pub fn parse_world_id(json: &Value) -> Result<u8, CensusError> {
    return try_parse_to::<u8>("world_id", json);
}

pub fn parse_zone_id(json: &Value) -> Result<u32, CensusError> {
    return try_parse_to::<u32>("world_id", json);
}
