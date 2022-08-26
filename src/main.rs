use listeners::{Listener, UnknownListener};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use widgets::{UnknownWidget, Widget};

mod data;
mod listeners;
mod widgets;

#[tokio::main]
async fn main() {
    init_log();

    let body = serde_json::from_reader(std::io::stdin());
    if let Ok(request) = body {
        match request {
            Request::Widget(widget) => print!("{}", widget.handle()),
            Request::NotManagedListener(l) => panic!("Unknown listener {}", l.action),
            Request::Listener(listener) => listener.handle().await,
            Request::NotManagedWidget(w) => panic!("Unknown widget {}", w.widget),
            Request::Other(_) => print!("{}", handle_manifest()),
        }
    } else {
        print!("{}", handle_manifest());
    }
}

fn init_log() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")
        .expect("Failed building the log file");
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("Failed building the log config");

    log4rs::init_config(config).expect("Failed applying the log config");
}

/** The application input */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Request {
    Listener(Listener),
    NotManagedListener(UnknownListener),
    Widget(Widget),
    NotManagedWidget(UnknownWidget),
    Other(Value),
}

fn handle_manifest() -> Value {
    json!({
        "manifest": {
            "widgets": ["root"],
            "listeners": [],
            "rootWidget": "root"
        }
    })
}
