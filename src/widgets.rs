use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{data::{service::Data, Counter}, listeners::{USER_DATASTORE, COUNTER_DATASTORE}};

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
#[serde(tag = "widget", rename_all = "camelCase")]
pub enum Widget {
    Root(BaseWidget),
    Menu(BaseWidget),
    Home(BaseWidget),
    Counter(CounterWidget),
}

impl Widget {
    pub fn handle(&self) -> Value {
        log::debug!("Widget: {:?}", self);
        let ret = match self {
            Widget::Root(_) => root(),
            Widget::Menu(_) => menu(),
            Widget::Home(_) => home(),
            Widget::Counter(counter_widget) => {
                let counter_option = counter_widget.data.get(0);
                if let Some(c) = counter_option {
                    if c.count.is_some() {
                        return counter(c, counter_widget.props.text.clone());
                    }
                }

                loading()
            }
        };
        log::debug!("Return: {}", ret);
        ret
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Decoration {
    pub color: Option<u32>,
    pub box_shadow: Option<BoxShadow>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxShadow {
    pub blur_radius: Option<u16>,
    pub color: Option<u32>,
    pub offset: Option<Offset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Offset {
    dx: u16,
    dy: u16,
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
          "name": "home"
        }
      ]
    })
}

fn menu() -> Value {
    json!({
        "type": "container",
        "decoration": Decoration {
            color: Some(0xFFFFFFFF),
            box_shadow: Some(BoxShadow {
                blur_radius: Some(8),
                color: Some(0x1A000000),
                offset: Some(Offset{
                    dx: 0,
                    dy: 1
                })
            }),
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

fn home() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "spacing": 4,
      "mainAxisAlignment": "spaceEvenly",
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "image",
          "src": "logo-vertical.png"
        },
        {
          "type": "widget",
          "name": "counter",
          "query": {
            "$find": {
              "_datastore": USER_DATASTORE,
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
              "_datastore": COUNTER_DATASTORE,
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
          "value": format!("{}: {}", text, data.count.unwrap_or(0))
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
