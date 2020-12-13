use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use crate::errors::ApiError;
use actix_web::web::Json;

pub const SUCCESS_MSG: &str = "Success";

pub fn time_now() -> i64 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
}

// move to web-commons?

pub type DynamicResult = Result<Json<HashMap<String, String>>, ApiError>;

pub fn make_api_response(success: bool, data: Vec<(String, String)>) -> HashMap<String, String> {
    let result = String::from(if success { SUCCESS_MSG } else { "error" });
    let mut map = HashMap::new();
    map.insert(String::from("result"), result);
    for d in data {
        map.insert(d.0, d.1);
    }

    return map;
}
