use serde_json::{json, Value};

use crate::widgets::{Padding, Decoration, BoxShadow, Offset, padding_symmetric};

pub fn menu() -> Value {
    let menu_content = json!({
        "type": "flex",
        "fillParent": true,
        "mainAxisAlignment": "spaceBetween",
        "crossAxisAlignment": "center",
        "padding": Padding { right: 4, ..Default::default() },
        "children": [
            {
              "type": "container",
              "constraints": {
                "minWidth": 32,
                "minHeight": 32,
                "maxWidth": 32,
                "maxHeight": 32,
              },
              "child": {
                "type": "image",
                "src": "logo.png"
              },
            },
            {
              "type": "flexible",
              "child": {
                "type": "container",
                "child": {
                  "type": "text",
                  "value": "Hello World",
                  "textAlign": "center",
                  "style": {
                    "fontWeight": "bold",
                    "fontSize": 24,
                  },
                }
              }
            }
        ]
    });
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
        "padding": padding_symmetric(2, 4),
        "child": menu_content,
    })
}
