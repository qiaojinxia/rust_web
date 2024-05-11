use serde_json::Value;

pub fn extract_string(meta: &Value, key: &'static str) -> String {
    meta.get(key).map_or_else(|| "".to_string(), |v| v.as_str().unwrap_or("").to_string())
}

pub fn extract_bool(meta: &Value, key: &'static str) -> Option<bool> {
    meta.get(key).and_then(|v| v.as_bool())
}

pub fn extract_i32(meta: &Value, key: &'static str) -> Option<i32> {
    meta.get(key).and_then(|v| v.as_i64().map(|i| i as i32))
}

pub fn extract_json(meta: &Value, key: &'static str) -> Option<Value> {
    meta.get(key).map(|v| {
        Value::Array(
            v.as_array().unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_object())
                .map(|obj| {
                    Value::Object(
                        obj.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                    )
                })
                .collect(),
        )
    })
}

