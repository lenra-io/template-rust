use lenra_app::{view::View, Handler};

use crate::views::{counter::counter, home::home, main::main, menu::menu};

mod counter;
mod home;
mod main;
mod menu;

pub fn get_views() -> Vec<View> {
    vec![
        View::new("main", main),
        View::new("home", home),
        View::new("menu", menu),
        View::new("counter", counter),
    ]
}
