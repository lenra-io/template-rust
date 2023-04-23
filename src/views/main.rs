use lenra_app::{view::ViewParams, Result};
use serde_json::{json, Value};

pub fn main(_params: ViewParams) -> Result<Value> {
    Ok(json!({
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
    }))
}
