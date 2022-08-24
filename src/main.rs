use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    let body = serde_json::from_reader(std::io::stdin());
    if let Ok(input) = body {
        match input {
            Input::Widget(widget) => handle_widget(widget),
            Input::Listener(_) => print!(""),
            Input::Other(_) => handle_manifest(),
        }
    } else {
        handle_manifest()
    }
}

/** The application input */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Input {
    Listener(Listener),
    Widget(Widget),
    Other(Value),
}

/** The Dofigen configuration */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Listener {
    pub action: String,
    pub props: Value,
    pub event: Value,
    pub api: Value,
}

/** The Dofigen configuration */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Widget {
    pub widget: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Value>,
}

fn handle_manifest() {
    print!(
        "{}",
        json!({
            "manifest": {
                "widgets": ["root"],
                "listeners": [],
                "rootWidget": "root"
            }
        })
    );
}

fn handle_widget(widget: Widget) {
    print!(
        "{}",
        json!({
            "type": "text",
            "value": format!("My app with the widget '{}'", widget.widget)
        })
    );
}
