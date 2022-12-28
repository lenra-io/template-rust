use serde_json::{Value, json};

pub fn loading() -> Value {
    json!({
        "type": "text",
        "value": "Loading..."
    })
}