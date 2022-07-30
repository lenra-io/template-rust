use serde_json::{json, Value};
use std::io::Result;

fn main() -> Result<()> {
    handle_json(serde_json::from_reader(std::io::stdin().lock()).unwrap());
    Ok(())
}

fn handle_json(json_value: Value) {
    // TODO: implement app
    let ret: String = if json_value["widget"].is_string() {
        handle_widget(json_value["widget"].as_str().unwrap()).to_string()
    } else {
        panic!("crash and burn")
    };
    print!("{}", ret);
}

fn handle_widget(widget_name: &str) -> Value {
    return json!({
        "type": "text",
        "value": format!("My app with the widget '{}'", widget_name)
    });
}
