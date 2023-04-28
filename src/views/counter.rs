use lenra_app::api::Doc;
use lenra_app::view::ViewParams;
use lenra_app::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::data::Counter;

pub fn counter(params: ViewParams<Vec<Counter>, CounterViewProps>) -> Result<Value> {
    let counters = params.data.unwrap();
    let counter = counters.get(0).unwrap();
    let text = params.props.unwrap().text;
    Ok(json!({
      "type": "flex",
      "spacing": 16,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "text",
          "value": format!("{}: {}", text, counter.count)
        },
        {
          "type": "button",
          "text": "+",
          "onPressed": {
              "action": "increment",
              "props": {
                  "id": counter.id()
              }
          }
        }
      ]
    }))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterViewProps {
    pub text: String,
}
