use lenra_app::{view::View, Handler};

use crate::views::lenra::{counter::counter, home::home, main::main, menu::menu};

mod counter;
mod home;
mod main;
mod menu;

pub fn get_views() -> Vec<View> {
    vec![
        View::new("lenra:main", main),
        View::new("lenra:home", home),
        View::new("lenra:menu", menu),
        View::new("lenra:counter", counter),
    ]
}
