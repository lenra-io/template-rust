use serde_json::{json, Value};

pub fn root() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "scroll": true,
      "spacing": 4,
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "widget",
          "name": "menu",
        },
        {
          "type": "widget",
          "name": "home"
        }
      ]
    })
}
