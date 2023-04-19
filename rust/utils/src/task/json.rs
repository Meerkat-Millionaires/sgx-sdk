use std::str::FromStr;

use jsonpath_rust::{JsonPathFinder, JsonPathInst};
use serde_json::Value;

use crate::error::Error;

pub fn json_parse_task(json: Value, path: &str) -> Result<Value, Error> {
    let json_path = JsonPathInst::from_str(path).map_err(|err| Error::CustomMessage(err))?;
    let finder = JsonPathFinder::new(Box::new(json), Box::new(json_path));

    let result = finder.find();
    if result.is_null() {
        return Err(Error::CustomMessage(
            "json_parse didnt find any values".to_string(),
        ));
    }
    if result.is_array() && result.as_array().unwrap().len() == 1 {
        let item = result.as_array().unwrap().get(0).unwrap();
        return Ok(item.to_owned());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_task() {
        let json = Value::from_str(r#"{"first":{"second":[{"active":1},{"passive":1}]}}"#).unwrap();

        let path = "$.first.second[0].active";
        let value = json_parse_task(json.clone(), path).unwrap();
        assert_eq!(value.as_i64().unwrap(), 1);
        assert_eq!(value.to_string(), "1");

        let path = "$.no_field";

        let value = json_parse_task(json.clone(), path);
        assert!(value.is_err());
    }
}
