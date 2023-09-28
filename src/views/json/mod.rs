use lenra_app::{view::View, Handler};

use crate::views::json::counter::counter;

mod counter;

pub fn get_views() -> Vec<View> {
    vec![View::new("json:counter", counter)]
}
