use serde_json::{json, Value};

pub fn main() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "scroll": true,
      "spacing": 4,
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "view",
          "name": "menu",
        },
        {
          "type": "view",
          "name": "home"
        }
      ]
    })
}
