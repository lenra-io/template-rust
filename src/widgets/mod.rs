use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    data::Counter,
    widgets::{counter::counter, home::home, loading::loading, menu::menu, main::main},
};

mod counter;
mod home;
mod loading;
mod menu;
mod main;

/** Unknown widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UnknownWidget {
    pub widget: String,
    pub data: Option<Value>,
    pub props: Option<Value>,
    pub context: Option<Context>,
}

/** Lenra widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "widget", rename_all = "camelCase")]
pub enum Widget {
    Main(BaseWidget),
    Menu(BaseWidget),
    Home(BaseWidget),
    Counter(CounterWidget),
}

impl Widget {
    pub fn handle(&self) -> Value {
        log::debug!("Widget: {:?}", self);
        let ret = match self {
            Widget::Main(_) => main(),
            Widget::Menu(_) => menu(),
            Widget::Home(_) => home(),
            Widget::Counter(counter_widget) => {
                let counter_option = counter_widget.data.get(0);
                if let Some(c) = counter_option {
                    return counter(c, counter_widget.props.text.clone());
                }

                loading()
            }
        };
        log::debug!("Return: {}", ret);
        ret
    }
}

/** Base widget body */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BaseWidget {
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
pub struct CounterWidget {
    pub data: Vec<Counter>,
    pub props: CounterWidgetProps,
    pub context: Option<Context>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterWidgetProps {
    text: String,
}

/** Lenra widget padding */
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
