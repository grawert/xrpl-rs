use serde_json::Value;

pub fn to_json_skip_nulls<T: serde::Serialize>(value: &T) -> String {
    let mut json =
        serde_json::to_value(value).expect("Failed to serialize to JSON value");
    _remove_nulls(&mut json);
    serde_json::to_string(&json)
        .expect("Failed to serialize JSON value to string")
}

pub fn to_value_skip_nulls<T: serde::Serialize>(value: &T) -> Value {
    let mut json =
        serde_json::to_value(value).expect("Failed to serialize to JSON value");
    _remove_nulls(&mut json);
    json
}

fn _remove_nulls(value: &mut Value) {
    if let Value::Object(map) = value {
        map.retain(|_, v| !v.is_null());
        map.values_mut().for_each(_remove_nulls);
    } else if let Value::Array(arr) = value {
        arr.iter_mut().for_each(_remove_nulls);
    }
}
