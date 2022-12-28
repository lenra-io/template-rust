use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    data::Counter,
    views::{counter::counter, home::home, loading::loading, menu::menu, main::main},
};

mod counter;
mod home;
mod loading;
mod menu;
mod main;

/** Unknown view request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UnknownView {
    pub view: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Context>,
}

/** Lenra view request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "view", rename_all = "camelCase")]
pub enum View {
    Main(BaseView),
    Menu(BaseView),
    Home(BaseView),
    Counter(CounterView),
}

impl View {
    pub fn handle(&self) -> Value {
        log::debug!("View: {:?}", self);
        let ret = match self {
            View::Main(_) => main(),
            View::Menu(_) => menu(),
            View::Home(_) => home(),
            View::Counter(counter_view) => {
                let counter_option = counter_view.data.get(0);
                if let Some(c) = counter_option {
                    return counter(c, counter_view.props.text.clone());
                }

                loading()
            }
        };
        log::debug!("Return: {}", ret);
        ret
    }
}

/** Base view body */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BaseView {
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Context>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub screen_size: Option<ScreenSize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ScreenSize {
    pub width: Option<u16>,
    pub height: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(default)]
pub struct CounterView {
    pub data: Vec<Counter>,
    pub props: CounterViewProps,
    pub context: Option<Context>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterViewProps {
    text: String,
}

/** Lenra view padding */
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

fn padding_symmetric(vertical: u16, horizontal: u16) -> Padding {
    Padding {
        top: vertical,
        bottom: vertical,
        left: horizontal,
        right: horizontal,
    }
}
