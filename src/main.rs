use serde_json::Value;
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
            },
            Err(_) => {}
        };
        buffer.push_str("\n");
    }
    Ok(())
}

fn handle_json(json_value:Value) {
    println!("{}", json_value.to_string());
    // TODO: implement app
}