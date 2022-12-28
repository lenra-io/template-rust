use listeners::{Listener, UnknownListener};
use resources::Resource;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use views::{UnknownView, View};

mod data;
mod listeners;
mod resources;
mod views;

fn main() {
    env_logger::init();

    let body = serde_json::from_reader(std::io::stdin());
    if let Ok(request) = body {
        match request {
            Request::View(view) => print!("{}", view.handle()),
            Request::Listener(listener) => listener.handle(),
            Request::NotManagedView(v) => {
                log::error!("Not managed view '{}'", v.view);
                panic!("Unknown view {}", v.view)
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
    View(View),
    NotManagedView(UnknownView),
    Resource(Resource),
    Other(Value),
}

fn handle_manifest() -> Value {
    json!({
        "manifest": {
            "rootView": "main"
        }
    })
}
