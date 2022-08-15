use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    let input: Input = serde_json::from_reader(std::io::stdin()).unwrap();

    match input {
        Input::Widget(widget) => handle_widget(widget),
        Input::Listener(_) => todo!(),
    }
}

/** The application input */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Input {
    Listener(Listener),
    Widget(Widget),
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
    pub data: Value,
    pub props: Value,
    pub context: Value,
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
