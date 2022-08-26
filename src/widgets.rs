use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::data::{service::Data, Counter};

/** Unknown widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UnknownWidget {
    pub widget: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Value>,
}

/** Lenra widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "widget", rename_all = "kebab-case")]
pub enum Widget {
    Root(BaseWidget),
    Menu(BaseWidget),
    Counters(BaseWidget),
    Counter(CounterWidget),
}

impl Widget {
    pub fn handle(&self) -> Value {
        match self {
            Widget::Root(_) => root(),
            Widget::Menu(_) => menu(),
            Widget::Counters(_) => counters(),
            Widget::Counter(counter_widget) => {
                let counter_option = counter_widget.data.get(0);
                if let Some(c) = counter_option {
                    return counter(c, counter_widget.props.text.clone());
                }

                loading()
            }
        }
    }
}

/** Base widget body */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BaseWidget {
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct CounterWidget {
    pub data: Vec<Counter>,
    pub props: CounterWidgetProps,
    pub context: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterWidgetProps {
    text: String,
}

/** Lenra widget padding */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Padding {
    pub top: u16,
    pub bottom: u16,
    pub left: u16,
    pub right: u16,
}

fn loading() -> Value {
    json!({
        "type": "text",
        "value": "Loading..."
    })
}

fn root() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "scroll": true,
      "spacing": 4,
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "widget",
          "name": "menu"
        },
        {
          "type": "widget",
          "name": "counters"
        }
      ]
    })
}

fn menu() -> Value {
    json!({
        "type": "container",
        "decoration": {
            "color": "FFFFFFFF",
            "boxShadow": {
                "blurRadius": 8,
                "color": "1A000000",
                "offset": {
                    "dx": 0,
                    "dy": 1
                }
            },
        },
        "child": {
            "type": "flex",
            "fillParent": true,
            "mainAxisAlignment": "spaceBetween",
            "crossAxisAlignment": "center",
            "padding": padding_symmetric(4, 2),
            "children": []
        }
    })
}

fn counters() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "spacing": 4,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "widget",
          "name": "counter",
          "query": {
            "$find": {
              "_datastore": "user",
              "_id": "@me"
            }
          },
          "props": CounterWidgetProps { text: "My personnal counter".into() }
        },
        {
          "type": "widget",
          "name": "counter",
          "query": {
            "$find": {
              "_datastore": "common_counter",
            }
          },
          "props": CounterWidgetProps { text: "The common counter".into() }
        }
      ]
    })
}

fn counter(data: &Counter, text: String) -> Value {
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
                  "id": data.id(),
                  "datastore": data.datastore()
              }
          }
        }
      ]
    })
}

fn padding_symmetric(vertical: u16, horizontal: u16) -> Padding {
    Padding {
        top: vertical,
        bottom: vertical,
        left: horizontal,
        right: horizontal,
    }
}
