use serde_json::{json, Value};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        buffer.push_str(&line.unwrap());
        let p: Result<Value, _> = serde_json::from_str(&buffer);
        match p {
            Ok(json_val) => {
                handle_json(json_val);
                break;
            }
            Err(_) => {}
        };
        buffer.push_str("\n");
    }
    Ok(())
}

fn handle_json(json_value: Value) {
    // TODO: implement app
    let ret: Value = if json_value["widget"].is_string() {
        handle_widget(json_value["widget"].as_str().unwrap())
    } else {
        panic!("crash and burn")
    };
    print!("{}", ret.to_string());
}

fn handle_widget(widget_name: &str) -> Value {
    return json!({
        "type": "text",
        "value": format!("My app with the widget '{}'", widget_name)
    });
}
