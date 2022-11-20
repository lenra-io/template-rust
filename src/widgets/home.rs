use serde_json::{json, Value};

use crate::{listeners::{COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER}, widgets::CounterWidgetProps};

pub fn home() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "spacing": 16,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "widget",
          "name": "counter",
          "coll": COUNTER_COLLECTION,
          "query": {
            "user": CURRENT_USER
          },
          "props": CounterWidgetProps { text: "My personnal counter".into() }
        },
        {
          "type": "widget",
          "name": "counter",
          "coll": COUNTER_COLLECTION,
          "query": {
            "user": GLOBAL_USER
          },
          "props": CounterWidgetProps { text: "The common counter".into() }
        }
      ]
    })
}
