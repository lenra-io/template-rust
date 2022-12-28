use serde_json::{json, Value};

use crate::{listeners::{COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER}, views::CounterViewProps};

pub fn home() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "spacing": 16,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "view",
          "name": "counter",
          "coll": COUNTER_COLLECTION,
          "query": {
            "user": CURRENT_USER
          },
          "props": CounterViewProps { text: "My personnal counter".into() }
        },
        {
          "type": "view",
          "name": "counter",
          "coll": COUNTER_COLLECTION,
          "query": {
            "user": GLOBAL_USER
          },
          "props": CounterViewProps { text: "The common counter".into() }
        }
      ]
    })
}
