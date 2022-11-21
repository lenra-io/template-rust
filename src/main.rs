use listeners::{Listener, UnknownListener};
use resources::Resource;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use widgets::{UnknownWidget, Widget};

mod data;
mod listeners;
mod resources;
mod widgets;

fn main() {
    env_logger::init();

    let body = serde_json::from_reader(std::io::stdin());
    if let Ok(request) = body {
        match request {
            Request::Widget(widget) => print!("{}", widget.handle()),
            Request::Listener(listener) => listener.handle(),
            Request::NotManagedWidget(w) => {
                log::error!("Not managed widget '{}'", w.widget);
                panic!("Unknown widget {}", w.widget)
            }
            Request::NotManagedListener(l) => {
                log::warn!("Not managed listener '{:?}'", l);
                panic!("Unknown action {}", l.action)
            }
            Request::Resource(resource) => resource.handle(),
            Request::Other(_) => print!("{}", handle_manifest()),
        }
    } else {
        print!("{}", handle_manifest());
    }
}

/** The application input */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Request {
    Listener(Listener),
    NotManagedListener(UnknownListener),
    Widget(Widget),
    NotManagedWidget(UnknownWidget),
    Resource(Resource),
    Other(Value),
}

fn handle_manifest() -> Value {
    json!({
        "manifest": {
            "rootWidget": "main"
        }
    })
}
