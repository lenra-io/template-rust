use serde_json::{json, Value};

use crate::data::service::Doc;
use crate::data::Counter;

pub fn counter(data: &Counter, text: String) -> Value {
    json!({
      "type": "flex",
      "spacing": 2,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "text",
          "value": format!("{}: {}", text, data.count)
        },
        {
          "type": "button",
          "text": "+",
          "onPressed": {
              "action": "increment",
              "props": {
                  "id": data.id()
              }
          }
        }
      ]
    })
}
